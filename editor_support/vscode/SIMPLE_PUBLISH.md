# VS Code Extension - Basit Yayımlama

## Yöntem 1: VSCE ile Direct Publish

1. **Node.js yükle** (eğer yoksa): https://nodejs.org/
2. **VSCE yükle:**
   ```cmd
   npm install -g vsce
   ```
3. **Bu klasörde terminal aç:**
   ```cmd
   cd C:\Users\mturkoglu\swe\nova-lang\editor_support\vscode
   ```
4. **Doğrudan publish et:**
   ```cmd
   vsce login tw4
   vsce publish
   ```

## Yöntem 2: GitHub Releases ile

1. Extension'ı GitHub'a push et
2. GitHub Actions ile otomatik publish
3. Daha güvenli ve tracking'li

## Yöntem 3: OVX Package

Eğer VSCE çalışmazsa:

1. Extension dosyalarını ZIP'le
2. .ovx uzantısıyla kaydet
3. VS Code Marketplace'de manuel upload

## Mevcut Durum

- Extension development mode'da çalışıyor ✅
- Syntax highlighting aktif ✅
- .vsix dosyası oluşturuluyor ✅
- Sadece marketplace upload sorunu var

## Önerilen Çözüm

En basit yol VSCE kullanmak. Eğer Personal Access Token sorunu varsa:
1. Azure DevOps'da yeni PAT oluştur
2. Full access ver
3. vsce login ile bağlan