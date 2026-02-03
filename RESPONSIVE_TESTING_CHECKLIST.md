# Responsive Testing Checklist

This checklist helps verify responsive design functionality when making UI changes.

## Pre-Testing Setup

1. Open DevTools (F12 in most browsers)
2. Enable Device Toolbar (Ctrl+Shift+M / Cmd+Shift+M)
3. Set viewport to test sizes

## Test Viewports

### Primary Breakpoints
- [ ] **320px** (small breakpoint minimum)
- [ ] **375px** (common mobile width)
- [ ] **600px** (form layout breakpoint)
- [ ] **768px** (medium breakpoint - tablet)
- [ ] **1024px** (large breakpoint - desktop)
- [ ] **1440px** (large desktop)
- [ ] **1920px** (ultrawide)

## Screen-by-Screen Testing

### Setup Screen
- [ ] Container scales between 320px and 600px
- [ ] Form inputs remain usable at 320px
- [ ] PIN confirmation fields visible at all sizes
- [ ] Submit button maintains 44px min-height
- [ ] No horizontal scroll appears

### Unlock Screen
- [ ] Container scales between 320px and 600px
- [ ] PIN input maintains 44px min-height
- [ ] Submit button visible and clickable
- [ ] Error messages display correctly

### Vault Screen
- [ ] Container scales between 400px and 1200px
- [ ] Header buttons remain accessible
- [ ] Search input maintains min 200px width
- [ ] Table transforms to cards at <768px
- [ ] Table data labels appear correctly
- [ ] Action buttons in table/cards work

### Add/Edit Key Modal
- [ ] Modal width scales to `min(90vw, 500px)`
- [ ] All form fields visible at 320px
- [ ] Modal doesn't exceed 85vh height
- [ ] Modal body scrolls when content overflows
- [ ] Close button accessible (44x44px)

### Delete Confirmation Modal
- [ ] Modal remains centered
- [ ] Both action buttons visible
- [ ] Warning text readable at all sizes

## Component Testing

### Typography
- [ ] h1 scales from 28px to 48px
- [ ] h2 scales from 24px to 36px
- [ ] h3 scales from 20px to 28px
- [ ] Body text scales from 16px to 18px
- [ ] Small text scales from 14px to 16px
- [ ] No text becomes too small to read

### Spacing
- [ ] Container padding scales responsively
- [ ] Form field gaps adjust appropriately
- [ ] Card grid gaps scale properly
- [ ] No excessive whitespace on large screens
- [ ] No cramped spacing on small screens

### Buttons
- [ ] All buttons maintain 44x44px minimum
- [ ] Button padding scales responsively
- [ ] Primary/secondary/danger styling consistent
- [ ] Icon buttons properly sized
- [ ] Hover states work at all sizes

### Form Inputs
- [ ] Text inputs maintain 200px min-width
- [ ] Inputs maintain 44px min-height
- [ ] Focus rings visible at all breakpoints
- [ ] Labels properly associated with inputs
- [ ] Error messages display correctly

### Table (≥768px)
- [ ] All columns visible
- [ ] Horizontal scroll appears if needed
- [ ] Cell padding appropriate
- [ ] Action buttons accessible

### Table (<768px)
- [ ] Transforms to stacked card layout
- [ ] Table headers hidden
- [ ] Data labels appear via `::before`
- [ ] Each row shows as a card
- [ ] Action buttons at bottom of card
- [ ] Cards have appropriate padding

## Accessibility Testing

### Touch Targets
- [ ] All buttons ≥44x44px
- [ ] All inputs ≥44px height
- [ ] Touch targets don't overlap
- [ ] Adequate spacing between interactive elements

### Keyboard Navigation
- [ ] Tab order logical at all viewports
- [ ] Focus rings visible
- [ ] All interactive elements reachable via keyboard
- [ ] Escape key closes modals
- [ ] Enter key submits forms

### Screen Readers
- [ ] aria-labels present on icon-only buttons
- [ ] Form labels properly associated
- [ ] Table headers marked correctly
- [ ] Modal announcements work
- [ ] Error messages accessible

### Contrast
- [ ] Primary text meets WCAG AA (4.5:1)
- [ ] Heading text meets WCAG AA
- [ ] UI elements have adequate contrast
- [ ] Focus indicators visible

## Interaction Testing

### Window Resize
- [ ] Layout adapts smoothly when resizing
- [ ] No layout shifts/jumps
- [ ] No horizontal scroll (unless intentional)
- [ ] Table/cards transform smoothly

### Modal Behavior
- [ ] Modals center at all viewport sizes
- [ ] Modal backdrop prevents background interaction
- [ ] Modal close buttons accessible
- [ ] Tab focus trapped within modal

## Performance Testing

- [ ] No janky animations during resize
- [ ] Layout recalculation is fast
- [ ] No excessive repaints/reflows
- [ ] Scrolling is smooth

## Browser Compatibility

Test in target browsers:
- [ ] Windows: WebView2 (Edge Chromium)
- [ ] macOS: WebKit (Safari technology)
- [ ] Linux: WebKitGTK

## Common Issues to Watch For

1. **Horizontal Scroll**: Content wider than viewport
2. **Text Truncation**: Text cut off before line wrapping
3. **Overlapping Elements**: Elements covering each other
4. **Too Small**: Text/buttons below minimum size
5. **Too Much Whitespace**: Excessive padding on large screens
6. **Touch Targets**: Elements too small for touch interaction
7. **Focus Loss**: Can't see which element has focus
8. **Modal Issues**: Modal too large/small or poorly positioned

## Regression Testing

After making changes, re-test:
1. All breakpoints
2. All screens
3. All interactive elements
4. Accessibility features
5. Modal behavior

## Pass Criteria

A change passes responsive testing if:
- ✅ No horizontal scroll at minimum viewport (320px)
- ✅ All touch targets ≥44x44px
- ✅ No text below 14px
- ✅ Focus rings visible on all interactive elements
- ✅ Layout transforms properly at all breakpoints
- ✅ Keyboard navigation works at all sizes
