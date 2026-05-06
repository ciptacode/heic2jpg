# @ciptacode/heic2jpg

HEIC to JPG/PNG converter using Rust and WebAssembly.

## Installation

```bash
npm install @ciptacode/heic2jpg
```

## Usage in React (Vite)

If you are using Vite, you can use the `web` target build.

### 1. Build the package

```bash
npm run build
```

### 2. Use in your React component

```tsx
import React, { useState } from 'react';
import init, { convert_heic_to_jpg } from '@ciptacode/heic2jpg';

function App() {
  const [resultImage, setResultImage] = useState<string | null>(null);

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    // Initialize the wasm module
    await init();

    const reader = new FileReader();
    reader.onload = async (event) => {
      const arrayBuffer = event.target?.result as ArrayBuffer;
      const uint8Array = new Uint8Array(arrayBuffer);

      try {
        // Convert HEIC to JPG (quality 80)
        const jpgData = convert_heic_to_jpg(uint8Array, 80);
        
        // Create a blob and URL for the result
        // Note: Casting to 'any' or 'BlobPart' might be needed if your TS version 
        // complains about Uint8Array vs BlobPart compatibility.
        const blob = new Blob([jpgData as any], { type: 'image/jpeg' });
        const url = URL.createObjectURL(blob);
        setResultImage(url);
      } catch (err) {
        console.error("Conversion failed", err);
      }
    };
    reader.readAsArrayBuffer(file);
  };

  return (
    <div>
      <h1>HEIC to JPG Converter</h1>
      <input type="file" accept=".heic" onChange={handleFileChange} />
      {resultImage && (
        <div>
          <h2>Result:</h2>
          <img src={resultImage} alt="Converted" style={{ maxWidth: '100%' }} />
          <br />
          <a href={resultImage} download="converted.jpg">Download JPG</a>
        </div>
      )}
    </div>
  );
}

export default App;
```

## Usage in Vanilla JS

You can also use it directly in the browser using ES Modules.

```html
<!DOCTYPE html>
<html lang="en">
<body>
  <input type="file" id="upload" accept=".heic">
  <div id="result"></div>

  <script type="module">
    // Import from the built package
    import init, { convert_heic_to_jpg } from './pkg/heic2jpg.js';

    async function run() {
      // Initialize the wasm module
      await init();

      document.getElementById('upload').addEventListener('change', async (e) => {
        const file = e.target.files[0];
        if (!file) return;

        const arrayBuffer = await file.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        try {
          // Convert HEIC to JPG (quality 80)
          const jpgData = convert_heic_to_jpg(uint8Array, 80);
          
          const blob = new Blob([jpgData], { type: 'image/jpeg' });
          const url = URL.createObjectURL(blob);

          const resultDiv = document.getElementById('result');
          resultDiv.innerHTML = `
            <h2>Result:</h2>
            <img src="${url}" style="max-width: 100%">
            <br>
            <a href="${url}" download="converted.jpg">Download JPG</a>
          `;
        } catch (err) {
          console.error("Conversion failed", err);
        }
      });
    }

    run();
  </script>
</body>
</html>
```

## API

### `convert_heic_to_jpg(data: Uint8Array, quality: number): Uint8Array`

Converts HEIC data to JPEG.
- `data`: Uint8Array of the HEIC file.
- `quality`: JPEG quality (1-100).
- Returns: Uint8Array of the JPEG file.

### `convert_heic_to_png(data: Uint8Array): Uint8Array`

Converts HEIC data to PNG.
- `data`: Uint8Array of the HEIC file.
- Returns: Uint8Array of the PNG file.

## Development

### Build for Web

```bash
npm run build
```

### Build for Bundlers (Webpack, etc.)

```bash
npm run build:bundler
```
