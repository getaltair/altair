import { readFileSync } from 'fs';
import { resolve } from 'path';
import { describe, it, expect } from 'vitest';

const CSS_PATH = resolve(__dirname, 'app.css');
const HTML_PATH = resolve(__dirname, 'app.html');

const css = readFileSync(CSS_PATH, 'utf-8');
const html = readFileSync(HTML_PATH, 'utf-8');

describe('app.css — Ethereal Canvas design system tokens', () => {
  it('does not contain #000000', () => {
    expect(css).not.toContain('#000000');
  });

  it('defines --on-surface with value #2a3435', () => {
    expect(css).toMatch(/--on-surface:\s*#2a3435/);
  });

  it('defines --font-display token', () => {
    expect(css).toContain('--font-display');
  });

  it('defines --font-body token', () => {
    expect(css).toContain('--font-body');
  });

  it('defines --motion-standard with 300ms cubic-bezier', () => {
    expect(css).toMatch(/--motion-standard:\s*300ms cubic-bezier/);
  });
});

describe('app.html — Google Fonts links', () => {
  it('contains a preconnect link for fonts.googleapis.com', () => {
    expect(html).toContain('https://fonts.googleapis.com');
  });

  it('contains a preconnect link for fonts.gstatic.com', () => {
    expect(html).toContain('https://fonts.gstatic.com');
  });

  it('contains a stylesheet link loading Manrope', () => {
    expect(html).toContain('Manrope');
  });

  it('contains a stylesheet link loading Plus Jakarta Sans', () => {
    expect(html).toContain('Plus+Jakarta+Sans');
  });
});
