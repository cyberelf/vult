const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// State
let currentKeyId = null;
let allKeys = [];

// DOM Elements
const setupScreen = document.getElementById('setup-screen');
const unlockScreen = document.getElementById('unlock-screen');
const vaultScreen = document.getElementById('vault-screen');
const setupForm = document.getElementById('setup-form');
const unlockForm = document.getElementById('unlock-form');
const keysList = document.getElementById('keys-list');
const emptyState = document.getElementById('empty-state');
const keyModal = document.getElementById('key-modal');
const viewKeyModal = document.getElementById('view-key-modal');
const keyForm = document.getElementById('key-form');

// Utility
function showError(elementId, message) {
    const el = document.getElementById(elementId);
    el.textContent = message;
    el.classList.remove('hidden');
    setTimeout(() => el.classList.add('hidden'), 5000);
}

function hideError(elementId) {
    document.getElementById(elementId).classList.add('hidden');
}

function showScreen(screen) {
    [setupScreen, unlockScreen, vaultScreen].forEach(s => s.classList.add('hidden'));
    screen.classList.remove('hidden');
}

// Initialize app
async function init() {
    try {
        const response = await invoke('get_auth_state');
        if (response.success) {
            if (!response.data.is_initialized) {
                showScreen(setupScreen);
            } else if (!response.data.is_unlocked) {
                showScreen(unlockScreen);
            } else {
                showScreen(vaultScreen);
                loadKeys();
            }
        }
    } catch (error) {
        console.error('Failed to check auth state:', error);
    }

    // Listen for auto-lock event
    listen('vault-auto-locked', () => {
        showScreen(unlockScreen);
    });

    // Start auto-lock checker
    setInterval(async () => {
        try {
            const response = await invoke('check_auto_lock');
            if (response.success && response.data) {
                showScreen(unlockScreen);
            }
        } catch (error) {
            console.error('Auto-lock check failed:', error);
        }
    }, 10000);
}

// Setup
setupForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    hideError('setup-error');

    const pin = document.getElementById('setup-pin').value;
    const confirm = document.getElementById('setup-pin-confirm').value;

    if (pin !== confirm) {
        showError('setup-error', 'PINs do not match');
        return;
    }

    try {
        const response = await invoke('init_vault', { pin });
        if (response.success) {
            showScreen(vaultScreen);
            loadKeys();
        } else {
            showError('setup-error', response.error || 'Failed to create vault');
        }
    } catch (error) {
        showError('setup-error', error.toString());
    }
});

// Unlock
unlockForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    hideError('unlock-error');

    const pin = document.getElementById('unlock-pin').value;

    try {
        const response = await invoke('unlock_vault', { pin });
        if (response.success) {
            showScreen(vaultScreen);
            loadKeys();
        } else {
            showError('unlock-error', response.error || 'Invalid PIN');
        }
    } catch (error) {
        showError('unlock-error', 'Invalid PIN');
    }
});

// Lock
document.getElementById('lock-btn').addEventListener('click', async () => {
    try {
        await invoke('lock_vault');
        showScreen(unlockScreen);
        document.getElementById('unlock-pin').value = '';
    } catch (error) {
        console.error('Failed to lock:', error);
    }
});

// Load Keys
async function loadKeys() {
    try {
        await invoke('update_activity');
        const response = await invoke('list_api_keys');
        if (response.success) {
            allKeys = response.data;
            renderKeys(allKeys);
        }
    } catch (error) {
        console.error('Failed to load keys:', error);
    }
}

// Render Keys
function renderKeys(keys) {
    if (keys.length === 0) {
        keysList.innerHTML = '';
        emptyState.classList.remove('hidden');
        return;
    }

    emptyState.classList.add('hidden');
    keysList.innerHTML = keys.map(key => `
        <div class="key-card" data-id="${key.id}">
            <div class="key-card-header">
                <span class="key-app-name">${escapeHtml(key.app_name)}</span>
                <span class="key-key-name">${escapeHtml(key.key_name)}</span>
            </div>
            ${key.description ? `<div class="key-description">${escapeHtml(key.description)}</div>` : ''}
        </div>
    `).join('');

    // Add click handlers
    document.querySelectorAll('.key-card').forEach(card => {
        card.addEventListener('click', () => viewKey(card.dataset.id));
    });
}

// Search
document.getElementById('search-input').addEventListener('input', async (e) => {
    const query = e.target.value.trim();
    try {
        await invoke('update_activity');
        if (query) {
            const response = await invoke('search_api_keys', { query });
            if (response.success) {
                renderKeys(response.data);
            }
        } else {
            renderKeys(allKeys);
        }
    } catch (error) {
        console.error('Search failed:', error);
    }
});

// Add Key Modal
document.getElementById('add-key-btn').addEventListener('click', () => {
    currentKeyId = null;
    document.getElementById('modal-title').textContent = 'Add API Key';
    keyForm.reset();
    keyModal.classList.remove('hidden');
});

// Close Modal
document.getElementById('close-modal').addEventListener('click', () => {
    keyModal.classList.add('hidden');
});

document.getElementById('cancel-key-btn').addEventListener('click', () => {
    keyModal.classList.add('hidden');
});

// Save Key
keyForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    hideError('key-form-error');

    const data = {
        appName: document.getElementById('app-name').value,
        keyName: document.getElementById('key-name').value,
        apiUrl: document.getElementById('api-url').value || null,
        description: document.getElementById('description').value || null,
        keyValue: document.getElementById('key-value').value,
    };

    try {
        await invoke('update_activity');
        let response;
        if (currentKeyId) {
            response = await invoke('update_api_key', {
                input: { id: currentKeyId, ...data }
            });
        } else {
            response = await invoke('create_api_key', { input: data });
        }

        if (response.success) {
            keyModal.classList.add('hidden');
            loadKeys();
        } else {
            showError('key-form-error', response.error || 'Failed to save key');
        }
    } catch (error) {
        showError('key-form-error', error.toString());
    }
});

// View Key
async function viewKey(id) {
    try {
        await invoke('update_activity');
        const response = await invoke('get_api_key', { id });
        if (response.success) {
            const key = response.data;
            currentKeyId = id;

            document.getElementById('view-app-name').textContent = key.api_key.app_name;
            document.getElementById('view-key-name').textContent = key.api_key.key_name;
            document.getElementById('view-api-url').textContent = key.api_key.api_url || 'N/A';
            document.getElementById('view-description').textContent = key.api_key.description || 'N/A';

            const valueEl = document.getElementById('view-key-value');
            valueEl.textContent = '••••••••••••';
            valueEl.classList.add('masked');
            valueEl.dataset.actualValue = key.key_value;

            viewKeyModal.classList.remove('hidden');
        }
    } catch (error) {
        console.error('Failed to load key:', error);
    }
}

// Close View Modal
document.getElementById('close-view-modal').addEventListener('click', () => {
    viewKeyModal.classList.add('hidden');
});

document.getElementById('close-view-btn').addEventListener('click', () => {
    viewKeyModal.classList.add('hidden');
});

// Reveal Key
document.getElementById('reveal-btn').addEventListener('click', () => {
    const valueEl = document.getElementById('view-key-value');
    const btn = document.getElementById('reveal-btn');

    if (valueEl.classList.contains('masked')) {
        valueEl.textContent = valueEl.dataset.actualValue;
        valueEl.classList.remove('masked');
        btn.textContent = 'Hide';
    } else {
        valueEl.textContent = '••••••••••••';
        valueEl.classList.add('masked');
        btn.textContent = 'Reveal';
    }
});

// Copy Key
document.getElementById('copy-btn').addEventListener('click', async () => {
    try {
        await invoke('update_activity');
        const response = await invoke('copy_to_clipboard', { id: currentKeyId });
        if (response.success) {
            const btn = document.getElementById('copy-btn');
            const originalText = btn.textContent;
            btn.textContent = 'Copied!';
            setTimeout(() => btn.textContent = originalText, 2000);
        }
    } catch (error) {
        console.error('Failed to copy:', error);
    }
});

// Edit Key
document.getElementById('edit-key-btn').addEventListener('click', async () => {
    try {
        await invoke('update_activity');
        const response = await invoke('get_api_key', { id: currentKeyId });
        if (response.success) {
            const key = response.data;
            viewKeyModal.classList.add('hidden');

            document.getElementById('modal-title').textContent = 'Edit API Key';
            document.getElementById('app-name').value = key.api_key.app_name;
            document.getElementById('key-name').value = key.api_key.key_name;
            document.getElementById('api-url').value = key.api_key.api_url || '';
            document.getElementById('description').value = key.api_key.description || '';
            document.getElementById('key-value').value = key.key_value;

            keyModal.classList.remove('hidden');
        }
    } catch (error) {
        console.error('Failed to load key for editing:', error);
    }
});

// Delete Key
document.getElementById('delete-key-btn').addEventListener('click', async () => {
    if (!confirm('Are you sure you want to delete this API key? This action cannot be undone.')) {
        return;
    }

    try {
        await invoke('update_activity');
        const response = await invoke('delete_api_key', { id: currentKeyId });
        if (response.success) {
            viewKeyModal.classList.add('hidden');
            loadKeys();
        }
    } catch (error) {
        console.error('Failed to delete key:', error);
    }
});

// Utility
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Activity tracking
document.addEventListener('click', () => {
    invoke('update_activity').catch(() => {});
});
document.addEventListener('keypress', () => {
    invoke('update_activity').catch(() => {});
});

// Init
init();
