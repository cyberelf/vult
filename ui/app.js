const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// State
let allKeys = [];
let keyVisibility = {}; // Track which keys are visible
let keyEditStates = {}; // Track which rows are being edited
let keyData = {}; // Store decrypted key values

// DOM Elements
const setupScreen = document.getElementById('setup-screen');
const unlockScreen = document.getElementById('unlock-screen');
const vaultScreen = document.getElementById('vault-screen');
const setupForm = document.getElementById('setup-form');
const unlockForm = document.getElementById('unlock-form');
const keysList = document.getElementById('keys-list');
const emptyState = document.getElementById('empty-state');
const keyModal = document.getElementById('key-modal');
const deleteModal = document.getElementById('delete-modal');
const keyForm = document.getElementById('key-form');
const keysTable = document.getElementById('keys-table');

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

function escapeHtml(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
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
        // Clear all decrypted data
        keyData = {};
        keyVisibility = {};
        keyEditStates = {};
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
            // Reset state for new keys
            allKeys.forEach(key => {
                if (!(key.id in keyVisibility)) {
                    keyVisibility[key.id] = false;
                }
                if (!(key.id in keyEditStates)) {
                    keyEditStates[key.id] = false;
                }
            });
            renderKeys();
        }
    } catch (error) {
        console.error('Failed to load keys:', error);
    }
}

// Render Keys as Table
function renderKeys() {
    if (allKeys.length === 0) {
        keysList.innerHTML = '';
        keysTable.classList.add('hidden');
        emptyState.classList.remove('hidden');
        return;
    }

    emptyState.classList.add('hidden');
    keysTable.classList.remove('hidden');

    keysList.innerHTML = allKeys.map(key => {
        const isEditing = keyEditStates[key.id] || false;
        const isVisible = keyVisibility[key.id] || false;
        const keyValue = isVisible ? (keyData[key.id] || 'Loading...') : '••••••••••••';

        if (isEditing) {
            // Render editable row
            return `
                <tr class="key-row editing" data-id="${key.id}">
                    <td><input type="text" class="edit-input" data-field="key_name" value="${escapeHtml(key.key_name)}"></td>
                    <td><input type="text" class="edit-input" data-field="app_name" value="${escapeHtml(key.app_name || '')}" placeholder="Optional"></td>
                    <td><input type="text" class="edit-input" data-field="api_url" value="${escapeHtml(key.api_url || '')}"></td>
                    <td><input type="text" class="edit-input" data-field="description" value="${escapeHtml(key.description || '')}"></td>
                    <td>
                        <input type="password" class="edit-input" data-field="key_value" value="${escapeHtml(keyData[key.id] || '')}" placeholder="Enter new value or leave unchanged">
                    </td>
                    <td class="actions-cell">
                        <button class="btn-icon btn-save" title="Save" onclick="saveRow('${key.id}')">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"/></svg>
                        </button>
                        <button class="btn-icon btn-cancel" title="Cancel" onclick="cancelEdit('${key.id}')">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12 19 6.41z"/></svg>
                        </button>
                    </td>
                </tr>
            `;
        } else {
            // Render read-only row
            return `
                <tr class="key-row" data-id="${key.id}">
                    <td>${escapeHtml(key.key_name)}</td>
                    <td>${escapeHtml(key.app_name || '-')}</td>
                    <td>${escapeHtml(key.api_url || '-')}</td>
                    <td>${escapeHtml(key.description || '-')}</td>
                    <td>
                        <span class="key-value ${isVisible ? '' : 'masked'}" data-id="${key.id}">${escapeHtml(keyValue)}</span>
                    </td>
                    <td class="actions-cell">
                        <button class="btn-icon btn-toggle-visibility" title="${isVisible ? 'Hide' : 'Show'}" onclick="toggleVisibility('${key.id}')">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                ${isVisible
                                    ? '<path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"/>'
                                    : '<path d="M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z"/>'
                                }
                            </svg>
                        </button>
                        <button class="btn-icon btn-copy" title="Copy Key" onclick="copyKey('${key.id}')">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/></svg>
                        </button>
                        <button class="btn-icon btn-edit" title="Edit" onclick="editRow('${key.id}')">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/></svg>
                        </button>
                        <button class="btn-icon btn-delete" title="Delete" onclick="showDeleteConfirm('${key.id}')">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
                        </button>
                    </td>
                </tr>
            `;
        }
    }).join('');
}

// Search
document.getElementById('search-input').addEventListener('input', async (e) => {
    const query = e.target.value.trim();
    try {
        await invoke('update_activity');
        if (query) {
            const response = await invoke('search_api_keys', { query });
            if (response.success) {
                allKeys = response.data;
                renderKeys();
            }
        } else {
            await loadKeys();
        }
    } catch (error) {
        console.error('Search failed:', error);
    }
});

// Toggle key visibility
async function toggleVisibility(id) {
    try {
        await invoke('update_activity');

        if (!keyVisibility[id]) {
            // Need to decrypt the key
            if (!keyData[id]) {
                const response = await invoke('get_api_key', { id });
                if (response.success) {
                    keyData[id] = response.data.key_value;
                }
            }
            keyVisibility[id] = true;
        } else {
            keyVisibility[id] = false;
        }
        renderKeys();
    } catch (error) {
        console.error('Failed to toggle visibility:', error);
        showError('toast', 'Failed to show key');
    }
}

// Copy key to clipboard
async function copyKey(id) {
    try {
        await invoke('update_activity');

        if (!keyData[id]) {
            const response = await invoke('get_api_key', { id });
            if (response.success) {
                keyData[id] = response.data.key_value;
            }
        }

        await navigator.clipboard.writeText(keyData[id]);

        // Show feedback
        const btn = document.querySelector(`[data-id="${id}"]`).closest('tr').querySelector('.btn-copy');
        const originalHTML = btn.innerHTML;
        btn.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"/></svg>';
        btn.classList.add('copied');
        setTimeout(() => {
            btn.innerHTML = originalHTML;
            btn.classList.remove('copied');
        }, 2000);
    } catch (error) {
        console.error('Failed to copy:', error);
        showError('toast', 'Failed to copy key');
    }
}

// Edit row
async function editRow(id) {
    try {
        await invoke('update_activity');

        // Load the key data if not already loaded
        if (!keyData[id]) {
            const response = await invoke('get_api_key', { id });
            if (response.success) {
                keyData[id] = response.data.key_value;
            }
        }

        keyEditStates[id] = true;
        renderKeys();
    } catch (error) {
        console.error('Failed to edit:', error);
        showError('toast', 'Failed to edit key');
    }
}

// Cancel edit
function cancelEdit(id) {
    keyEditStates[id] = false;
    renderKeys();
}

// Save row
async function saveRow(id) {
    try {
        await invoke('update_activity');

        const row = document.querySelector(`tr[data-id="${id}"]`);
        const key_name = row.querySelector('[data-field="key_name"]').value.trim();
        const app_name = row.querySelector('[data-field="app_name"]').value.trim() || null;
        const api_url = row.querySelector('[data-field="api_url"]').value.trim() || null;
        const description = row.querySelector('[data-field="description"]').value.trim() || null;
        const key_value = row.querySelector('[data-field="key_value"]').value.trim() || null;

        if (!key_name) {
            showError('toast', 'Key Name is required');
            return;
        }

        const updateData = {
            id: id,
            key_name: key_name,
        };

        // Only include optional fields if they have values
        if (app_name) {
            updateData.app_name = app_name;
        }
        if (api_url) {
            updateData.api_url = api_url;
        }
        if (description) {
            updateData.description = description;
        }
        if (key_value) {
            updateData.key_value = key_value;
        }

        const response = await invoke('update_api_key', { input: updateData });

        if (response.success) {
            keyEditStates[id] = false;
            // Update cached data
            if (key_value) {
                keyData[id] = key_value;
            }
            await loadKeys();
        } else {
            showError('toast', response.error || 'Failed to save key');
        }
    } catch (error) {
        console.error('Failed to save:', error);
        showError('toast', `Error: ${error}`);
    }
}

// Delete key
let keyToDelete = null;

function showDeleteConfirm(id) {
    keyToDelete = id;
    deleteModal.classList.remove('hidden');
}

document.getElementById('close-delete-modal').addEventListener('click', () => {
    deleteModal.classList.add('hidden');
    keyToDelete = null;
});

document.getElementById('cancel-delete-btn').addEventListener('click', () => {
    deleteModal.classList.add('hidden');
    keyToDelete = null;
});

document.getElementById('confirm-delete-btn').addEventListener('click', async () => {
    if (!keyToDelete) return;

    try {
        await invoke('update_activity');
        const response = await invoke('delete_api_key', { id: keyToDelete });

        if (response.success) {
            deleteModal.classList.add('hidden');
            // Clean up
            delete keyData[keyToDelete];
            delete keyVisibility[keyToDelete];
            delete keyEditStates[keyToDelete];
            keyToDelete = null;
            await loadKeys();
        } else {
            showError('toast', response.error || 'Failed to delete key');
        }
    } catch (error) {
        console.error('Failed to delete:', error);
        showError('toast', 'Failed to delete key');
    }
});

// Add Key Modal
document.getElementById('add-key-btn').addEventListener('click', () => {
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

    const key_name = document.getElementById('key-name').value.trim();
    const app_name = document.getElementById('app-name').value.trim() || null;
    const key_value = document.getElementById('key-value').value.trim();

    if (!key_name) {
        showError('key-form-error', 'Key name is required');
        return;
    }
    if (!key_value) {
        showError('key-form-error', 'API key value is required');
        return;
    }

    const data = {
        key_name: String(key_name),
        key_value: String(key_value),
    };

    // Only include optional fields if they have values
    if (app_name) {
        data.app_name = String(app_name);
    }

    const api_url = document.getElementById('api-url').value.trim();
    if (api_url) {
        data.api_url = String(api_url);
    }

    const description = document.getElementById('description').value.trim();
    if (description) {
        data.description = String(description);
    }

    try {
        await invoke('update_activity');
        const response = await invoke('create_api_key', {
            input: data
        });

        if (response.success) {
            keyModal.classList.add('hidden');
            loadKeys();
        } else {
            showError('key-form-error', response.error || 'Failed to save key');
        }
    } catch (error) {
        console.error('Full error:', error);
        showError('key-form-error', `Error: ${error}`);
    }
});

// Activity tracking
document.addEventListener('click', () => {
    invoke('update_activity').catch(() => {});
});
document.addEventListener('keypress', () => {
    invoke('update_activity').catch(() => {});
});

// Make functions globally available for onclick handlers
window.toggleVisibility = toggleVisibility;
window.copyKey = copyKey;
window.editRow = editRow;
window.cancelEdit = cancelEdit;
window.saveRow = saveRow;
window.showDeleteConfirm = showDeleteConfirm;

// Init
init();
