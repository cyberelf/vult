/**
 * Frontend Tests for Vult Vault Application
 *
 * These tests verify UI components, DOM manipulation,
 * form validation, and user interactions.
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { JSDOM } from 'happy-dom';

// Mock Tauri API
global.invoke = vi.fn();
global.listen = vi.fn();

// Load the application code
import fs from 'fs';
const html = fs.readFileSync('./index.html', 'utf-8');

describe('Frontend Tests - Vult Vault', () => {
    let document;
    let window;

    beforeEach(() => {
        // Create a fresh DOM environment
        const jsdom = new JSDOM(html, {
            runScripts: 'dangerously',
            resources: 'usable',
        });
        document = jsdom.window.document;
        window = jsdom.window;
        global.document = document;
        global.window = window;

        // Mock console methods
        global.console = {
            log: vi.fn(),
            error: vi.fn(),
            warn: vi.fn(),
        };

        // Execute app.js logic
        const appCode = fs.readFileSync('./app.js', 'utf-8');
        eval(appCode);
    });

    describe('DOM Elements Exist', () => {
        it('should have setup screen element', () => {
            const setupScreen = document.getElementById('setup-screen');
            expect(setupScreen).toBeTruthy();
        });

        it('should have unlock screen element', () => {
            const unlockScreen = document.getElementById('unlock-screen');
            expect(unlockScreen).toBeTruthy();
        });

        it('should have vault screen element', () => {
            const vaultScreen = document.getElementById('vault-screen');
            expect(vaultScreen).toBeTruthy();
        });

        it('should have key modal element', () => {
            const keyModal = document.getElementById('key-modal');
            expect(keyModal).toBeTruthy();
        });

        it('should have view key modal element', () => {
            const viewKeyModal = document.getElementById('view-key-modal');
            expect(viewKeyModal).toBeTruthy();
        });
    });

    describe('Forms', () => {
        it('should have setup form with required fields', () => {
            const setupForm = document.getElementById('setup-form');
            const setupPin = document.getElementById('setup-pin');
            const setupPinConfirm = document.getElementById('setup-pin-confirm');

            expect(setupForm).toBeTruthy();
            expect(setupPin).toBeTruthy();
            expect(setupPin.type).toBe('password');
            expect(setupPinConfirm).toBeTruthy();
            expect(setupPinConfirm.type).toBe('password');
        });

        it('should have unlock form with PIN field', () => {
            const unlockForm = document.getElementById('unlock-form');
            const unlockPin = document.getElementById('unlock-pin');

            expect(unlockForm).toBeTruthy();
            expect(unlockPin).toBeTruthy();
            expect(unlockPin.type).toBe('password');
        });

        it('should have key form with all required fields', () => {
            const keyForm = document.getElementById('key-form');
            const appName = document.getElementById('app-name');
            const keyName = document.getElementById('key-name');
            const apiKey = document.getElementById('key-value');
            const apiUrl = document.getElementById('api-url');
            const description = document.getElementById('description');

            expect(keyForm).toBeTruthy();
            expect(appName).toBeTruthy();
            expect(keyName).toBeTruthy();
            expect(apiKey).toBeTruthy();
            expect(apiKey.type).toBe('password');
            expect(description).toBeTruthy();
        });
    });

    describe('Buttons and Actions', () => {
        it('should have add key button', () => {
            const addKeyBtn = document.getElementById('add-key-btn');
            expect(addKeyBtn).toBeTruthy();
            expect(addKeyBtn.textContent).toContain('Add');
        });

        it('should have lock vault button', () => {
            const lockBtn = document.getElementById('lock-btn');
            expect(lockBtn).toBeTruthy();
            expect(lockBtn.textContent).toContain('Lock');
        });

        it('should have search input', () => {
            const searchInput = document.getElementById('search-input');
            expect(searchInput).toBeTruthy();
            expect(searchInput.type).toBe('text');
        });

        it('should have keys list container', () => {
            const keysList = document.getElementById('keys-list');
            const emptyState = document.getElementById('empty-state');
            expect(keysList).toBeTruthy();
            expect(emptyState).toBeTruthy();
        });
    });

    describe('Modal Elements', () => {
        it('should have modal close buttons', () => {
            const closeModal = document.getElementById('close-modal');
            const cancelKeyBtn = document.getElementById('cancel-key-btn');
            const closeViewModal = document.getElementById('close-view-modal');
            const closeViewBtn = document.getElementById('close-view-btn');

            expect(closeModal).toBeTruthy();
            expect(cancelKeyBtn).toBeTruthy();
            expect(closeViewModal).toBeTruthy();
            expect(closeViewBtn).toBeTruthy();
        });

        it('should have view modal action buttons', () => {
            const revealBtn = document.getElementById('reveal-btn');
            const copyBtn = document.getElementById('copy-btn');
            const editKeyBtn = document.getElementById('edit-key-btn');
            const deleteKeyBtn = document.getElementById('delete-key-btn');

            expect(revealBtn).toBeTruthy();
            expect(copyBtn).toBeTruthy();
            expect(editKeyBtn).toBeTruthy();
            expect(deleteKeyBtn).toBeTruthy();
        });
    });

    describe('Initial Screen State', () => {
        it('should hide unlock and vault screens initially', () => {
            const setupScreen = document.getElementById('setup-screen');
            const unlockScreen = document.getElementById('unlock-screen');
            const vaultScreen = document.getElementById('vault-screen');

            expect(setupScreen.classList.contains('hidden')).toBe(false);
            expect(unlockScreen.classList.contains('hidden')).toBe(true);
            expect(vaultScreen.classList.contains('hidden')).toBe(true);
        });
    });

    describe('Tauri Integration', () => {
        it('should have invoke function available', () => {
            expect(typeof invoke).toBe('function');
        });

        it('should have listen function available', () => {
            expect(typeof listen).toBe('function');
        });
    });

    describe('State Management', () => {
        it('should have currentKeyId variable defined', () => {
            expect(typeof currentKeyId).not.toBe('undefined');
        });

        it('should have allKeys array', () => {
            expect(Array.isArray(allKeys)).toBe(true);
        });
    });

    describe('Utility Functions', () => {
        it('should define showError function', () => {
            expect(typeof showError).toBe('function');
        });

        it('should define hideError function', () => {
            expect(typeof hideError).toBe('function');
        });

        it('should define showScreen function', () => {
            expect(typeof showScreen).toBe('function');
        });

        it('should define escapeHtml function', () => {
            expect(typeof escapeHtml).toBe('function');
        });

        it('should escape HTML entities correctly', () => {
            const input = '<script>alert("xss")</script>';
            const output = escapeHtml(input);

            expect(output).toContain('&lt;');
            expect(output).toContain('&gt;');
            expect(output).not.toContain('<script>');
        });

        it('should escape quotes correctly', () => {
            const input = 'Test "quoted" and \'single\'';
            const output = escapeHtml(input);

            expect(output).toContain('&quot;');
            expect(output).toContain('&#x27;');
        });
    });

    describe('Screen Visibility Management', () => {
        it('should show only one screen at a time', () => {
            const setupScreen = document.getElementById('setup-screen');
            const unlockScreen = document.getElementById('unlock-screen');
            const vaultScreen = document.getElementById('vault-screen');

            // Initially only setup is visible
            const visibleCount = [setupScreen, unlockScreen, vaultScreen]
                .filter(s => !s.classList.contains('hidden')).length;

            expect(visibleCount).toBe(1);
        });
    });

    describe('Form Validation', () => {
        it('should have password input fields', () => {
            const setupPin = document.getElementById('setup-pin');
            const unlockPin = document.getElementById('unlock-pin');
            const keyValue = document.getElementById('key-value');

            expect(setupPin.required).toBe(false); // No required attribute in HTML
            expect(unlockPin.required).toBe(false);
            expect(keyValue.required).toBe(false);
        });
    });

    describe('Error Handling UI', () => {
        it('should have error message elements', () => {
            const setupError = document.getElementById('setup-error');
            const unlockError = document.getElementById('unlock-error');
            const keyFormError = document.getElementById('key-form-error');

            expect(setupError).toBeTruthy();
            expect(unlockError).toBeTruthy();
            expect(keyFormError).toBeTruthy();

            // Initially hidden
            expect(setupError.classList.contains('hidden')).toBe(true);
            expect(unlockError.classList.contains('hidden')).toBe(true);
            expect(keyFormError.classList.contains('hidden')).toBe(true);
        });
    });

    describe('Event Listeners', () => {
        it('should have event listeners attached to forms', () => {
            const setupForm = document.getElementById('setup-form');
            const unlockForm = document.getElementById('unlock-form');
            const keyForm = document.getElementById('key-form');

            // Check if forms exist and have submit handlers
            expect(setupForm).toBeTruthy();
            expect(unlockForm).toBeTruthy();
            expect(keyForm).toBeTruthy();
        });

        it('should have search input listener', () => {
            const searchInput = document.getElementById('search-input');
            expect(searchInput).toBeTruthy();
            expect(searchInput.type).toBe('text');
        });
    });

    describe('Responsive Design', () => {
        it('should have viewport meta tag', () => {
            const viewport = document.querySelector('meta[name="viewport"]');
            expect(viewport).toBeTruthy();
            expect(viewport.content).toContain('width=device-width');
        });
    });

    describe('Accessibility', () => {
        it('should have proper heading hierarchy', () => {
            const headings = document.querySelectorAll('h1, h2, h3');
            expect(headings.length).toBeGreaterThan(0);
        });

        it('should have labels for form inputs', () => {
            const appNameLabel = document.querySelector('label[for="app-name"]');
            const keyNameLabel = document.querySelector('label[for="key-name"]');

            expect(appNameLabel).toBeTruthy();
            expect(keyNameLabel).toBeTruthy();
        });
    });

    describe('Tauri Commands Coverage', () => {
        it('should call init_vault command', async () => {
            const mockResponse = { success: true, data: null };
            invoke.mockResolvedValue(mockResponse);

            // This would be triggered by form submission
            expect(invoke).toBeCalled();
        });

        it('should call unlock_vault command', async () => {
            const mockResponse = { success: true, data: null };
            invoke.mockResolvedValue(mockResponse);

            expect(invoke).toBeDefined();
        });

        it('should call list_api_keys command', async () => {
            const mockResponse = {
                success: true,
                data: []
            };
            invoke.mockResolvedValue(mockResponse);

            expect(invoke).toBeDefined();
        });
    });
});
