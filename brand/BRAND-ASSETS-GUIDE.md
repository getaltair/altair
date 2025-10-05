# Altair Brand Assets - Complete Package

**Generated:** October 2025
**Version:** 1.0

---

## 📦 What's Included

This package contains everything you need to launch Altair with professional, consistent branding across all platforms.

---

## 🎨 Core Assets

### 1. Diamond Star Icon
**File:** `diamond-star.svg`

Your primary logo/icon - a geometric diamond-shaped star representing the Altair star.

**Usage:**
- Social media profile pictures
- App icon
- Website favicon (as base)
- GitHub repository avatar
- Documentation headers

**Quick setup:**
- Upload to Twitter/GitHub/LinkedIn as profile photo
- Use as base for favicon generation
- Reference in your README

---

### 2. Landing Page
**File:** `altair-landing-page.html`

A complete, production-ready landing page featuring:
- Hero section with icon and tagline
- Email signup form
- Feature highlights
- Clean, ADHD-friendly design
- Mobile responsive
- Dark theme matching brand

**How to use:**
1. Open the file and test it locally
2. Replace the form submission with your actual email service (Mailchimp, ConvertKit, etc.)
3. Update links to your actual GitHub/Twitter handles
4. Deploy to Cloudflare Pages, Netlify, or Vercel
5. Point your domain to it

**What to customize:**
- Line 85: Update the form submission handler
- Lines 157-159: Add your real social media links
- Add Google Analytics or Plausible if desired

---

### 3. README.md
**File:** `README.md`

Professional GitHub repository README with:
- Logo integration
- Project description
- Status badges
- Tech stack overview
- Contributing guidelines
- Roadmap timeline
- Community links

**How to use:**
1. Copy to your GitHub repository root
2. Upload the `diamond-star.svg` to `assets/icon.svg` in your repo
3. Update the icon URL in line 1
4. Customize the roadmap dates (lines 87-103)
5. Update social media handles

**What to customize:**
- Links (replace `getaltair` with your actual GitHub org)
- Timeline dates
- Tech stack details as you solidify decisions
- Add more sections as needed

---

### 4. Social Media Banners
**File:** `altair-social-banners.html`

Pre-designed banners for all major platforms:
- **Twitter/X Header** (1500×500px)
- **GitHub Social Preview** (1280×640px)
- **LinkedIn Banner** (1584×396px)
- **Discord Server Banner** (960×540px)

**How to use:**
1. Open the HTML file in your browser
2. Click download buttons to save each banner
3. Upload to respective platforms:
   - Twitter: Profile → Edit Profile → Header Photo
   - GitHub: Repository → Settings → Social Preview
   - LinkedIn: Profile → Banner Image
   - Discord: Server Settings → Overview → Banner

**All banners include:**
- Altair diamond star icon
- Gradient backgrounds
- Professional typography
- Proper sizing for each platform

---

### 5. Brand Guidelines
**File:** `BRAND-GUIDELINES.md`

Comprehensive 30-page brand manual covering:
- Logo usage rules
- Complete color palette with hex codes
- Typography specifications
- Voice and tone guidelines
- ADHD-friendly design principles
- Accessibility standards
- Example applications

**How to use:**
- Share with contributors and designers
- Reference when making design decisions
- Include in your docs folder
- Use as basis for style guide website

**Key sections:**
- Colors (page 1): Copy hex codes for your CSS
- Typography (page 2): Font weights and sizes
- Voice & Tone (page 3): Writing guidelines
- Accessibility (page 5): WCAG compliance

---

### 6. Favicon Generator
**File:** `altair-favicon-generator.html`

Interactive tool and instructions for creating favicons in all required formats.

**Includes:**
- Visual previews at 16px, 32px, 48px, 64px
- Three methods for favicon creation
- HTML code snippets
- PWA icon size requirements
- Testing instructions

**Quick start:**
1. Use Option 1 (Online Generator) - easiest
2. Go to favicon.io/favicon-converter
3. Upload your `diamond-star.svg`
4. Download the package
5. Add to your website root

**For modern sites:**
- Just use the SVG favicon (one line of HTML!)
- Works in all modern browsers
- Perfect quality at any size

---

## 🚀 Quick Start Checklist

### Immediate Actions (Today)

- [ ] Upload `diamond-star.svg` to Twitter as profile photo
- [ ] Upload to GitHub organization as avatar
- [ ] Set Twitter banner using social banners file
- [ ] Save all color codes from brand guidelines
- [ ] Bookmark brand guidelines for reference

### This Week

- [ ] Deploy landing page to getaltair.app
- [ ] Set up email signup (Mailchimp/ConvertKit)
- [ ] Create GitHub repository and add README
- [ ] Generate and add favicon to website
- [ ] Update all social media bios with consistent tagline

### Ongoing

- [ ] Reference brand guidelines for all design decisions
- [ ] Share guidelines with contributors
- [ ] Keep README updated with progress
- [ ] Post updates using consistent brand voice

---

## 📁 Recommended File Structure

```
/your-project/
  /brand/
    diamond-star.svg
    BRAND-GUIDELINES.md
    /banners/
      twitter-banner.svg
      github-preview.svg
      linkedin-banner.svg
      discord-banner.svg
    /favicons/
      favicon.ico
      favicon.svg
      (generated files)

  /website/
    index.html (landing page)
    /assets/
      /icons/
        diamond-star.svg

  README.md
```

---

## 🎯 Platform-Specific Instructions

### GitHub

1. **Organization Avatar:**
   - Settings → Profile → Picture
   - Upload `diamond-star-512.png`

2. **Repository Social Preview:**
   - Repository → Settings → Social Preview
   - Upload GitHub banner from social banners

3. **README:**
   - Copy `README.md` to repository root
   - Create `/assets/` folder and add icon
   - Update all links to match your org/repo names

### Twitter/X

1. **Profile Photo:** `diamond-star-512.png`
2. **Header:** Twitter banner from social banners
3. **Bio:** "Where focus takes flight 🌟 ADHD-friendly project management • Open Source • Built by the community"
4. **Handle:** @getaltair (or your chosen handle)

### LinkedIn

1. **Company Page Photo:** `diamond-star-512.png`
2. **Banner:** LinkedIn banner from social banners
3. **Tagline:** "ADHD-Friendly Project Management"
4. **Description:** Use about section from README

### Discord

1. **Server Icon:** `diamond-star-256.png`
2. **Server Banner:** Discord banner from social banners
3. **Server Description:** "Altair community - Building ADHD-friendly project management together"

---

## 🛠️ Tools You'll Need

### Required
- **Text Editor** - VS Code, Sublime, or similar
- **Web Browser** - Chrome, Firefox, or Edge
- **Image Viewer** - Preview (Mac), Photos (Windows)

### Recommended
- **Favicon Generator** - favicon.io (online, free)
- **Git** - For repository management
- **Figma** - For future design iterations (free)

### Optional
- **ImageMagick** - For advanced image processing
- **Inkscape** - For editing SVG files

---

## 📝 Customization Guide

### Colors
All colors are defined in brand guidelines. To customize:
1. Choose new hex codes
2. Update in CSS variables
3. Regenerate banners if needed
4. Maintain contrast ratios (4.5:1 minimum)

### Tagline
Current: "Where focus takes flight"

Alternatives you might consider:
- "Navigate your brightest path"
- "Guided by your brightest star"
- "Chart your course with clarity"

To change:
1. Update landing page (line 41)
2. Update README (line 3)
3. Update social media bios
4. Regenerate banners

### Typography
Using Inter font family. To change:
1. Update Google Fonts import
2. Modify CSS font-family
3. Update brand guidelines
4. Test for ADHD-friendly readability

---

## 🐛 Troubleshooting

### Icons not showing up
- Check file path is correct
- Ensure SVG file is uploaded
- Hard refresh browser (Ctrl+Shift+R)
- Verify file permissions

### Favicon not updating
- Clear browser cache
- Check file is in root directory
- Verify HTML link tags
- Wait 24 hours (browsers cache aggressively)

### Colors look different
- Check color profile (use sRGB)
- Verify hex codes match guidelines
- Test on multiple devices
- Consider display calibration

### Landing page form not working
- Add actual email service integration
- Check console for JavaScript errors
- Test with different email addresses
- Verify CORS settings if hosted

---

## 📞 Questions?

If you need help with any of these assets:
- Review the brand guidelines first
- Check individual file instructions
- Create a GitHub issue for bugs
- Email hello@getaltair.app for support

---

## ✅ Success Metrics

You'll know your branding is working when:
- [ ] All platforms have consistent visuals
- [ ] People recognize the diamond star icon
- [ ] Social media looks professional
- [ ] Contributors understand brand voice
- [ ] Website gets signups
- [ ] Community grows organically

---

## 🎉 What's Next?

After setting up these assets:

1. **Launch your landing page**
2. **Announce on social media**
3. **Start building in public**
4. **Share progress updates**
5. **Engage with ADHD community**
6. **Iterate based on feedback**

Remember: Perfect branding doesn't matter if you don't ship. Use these assets, launch, and improve as you go!

---

**Ready to launch? You've got everything you need. Let's build something amazing! 🚀**
