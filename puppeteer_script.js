const puppeteer = require('puppeteer');

(async () => {
    const browser = await puppeteer.launch();
    const page = await browser.newPage();
    await page.goto('https://canopy-how-it-works.notion.site/HOW-IT-WORKS-f6314627a8fa46f1852e9973af60876b', {
        waitUntil: 'networkidle2',
    });

    // Save the full page content as HTML
    const htmlContent = await page.content();
    const fs = require('fs');
    fs.writeFileSync('notion_full_page.html', htmlContent);

    // Optionally, extract and save PDFs, images, etc.
    // Example: Save a screenshot
    await page.screenshot({ path: 'notion_page.png', fullPage: true });

    await browser.close();
})();
