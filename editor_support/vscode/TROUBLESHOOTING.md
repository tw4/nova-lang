# VS Code Extension Publishing Troubleshooting

## Personal Access Token Error

### Problem
```
ERROR: The Personal Access Token verification has failed.
Error: Access Denied: needs the following permission(s) on the resource /tw4 to perform this action: View user permissions on a resource
```

### Çözüm 1: PAT İzinlerini Düzelt

1. **Azure DevOps'a git:** https://dev.azure.com/
2. **User Settings > Personal Access Tokens**
3. Mevcut token'ı düzenle veya yeni bir tane oluştur
4. **Gerekli izinler:**
   - Organization: `All accessible organizations`
   - Scopes: `Full access` (recommended) veya minimum:
     - `Marketplace: Manage`
     - `Identity: Read`
     - `User Profile: Read`

### Çözüm 2: Publisher Profili Kontrol Et

1. **Marketplace'e git:** https://marketplace.visualstudio.com/manage
2. **Publisher profili oluştur/kontrol et:**
   - Publisher ID: `tw4`
   - Display Name: `tw4`
3. Publisher profili yoksa yeni oluştur

### Çözüm 3: Manual Upload

Eğer token sorunu devam ederse, manual upload kullan:

1. **Extension'ı package'la:**
   ```bash
   cd editor_support/vscode
   vsce package
   ```

2. **Manual upload:**
   - https://marketplace.visualstudio.com/manage adresine git
   - "New extension" > "Visual Studio Code"
   - `nova-language-0.1.0.vsix` dosyasını upload et

### Çözüm 4: Alternatif PAT Oluşturma

1. **Yeni Azure DevOps Organization oluştur:**
   - https://dev.azure.com/
   - "New organization" oluştur
   - Organization name: `tw4-extensions`

2. **Bu organization için PAT oluştur:**
   - Scope: `All scopes` (full access)
   - Expiry: 1 year

3. **Yeni PAT ile login:**
   ```bash
   vsce login tw4
   ```

## Test ve Doğrulama

### Local Test
```bash
# Package oluştur
vsce package

# Local olarak test et
code --install-extension nova-language-0.1.0.vsix
```

### Marketplace'de Kontrol
Yayımlandıktan sonra:
- https://marketplace.visualstudio.com/items?itemName=tw4.nova-language

## Yaygın Hatalar

### 1. Publisher bulunamadı
- Marketplace'de publisher profili oluştur
- Publisher ID'nin unique olduğundan emin ol

### 2. Extension name çakışması
- `package.json`'da `name` alanını değiştir
- Unique bir isim seç

### 3. Token expired
- Yeni PAT oluştur
- `vsce login` komutunu tekrar çalıştır

## Başarılı Yayım İçin Checklist

- [ ] Azure DevOps hesabı var
- [ ] Marketplace publisher profili oluşturuldu (`tw4`)
- [ ] Full access PAT oluşturuldu
- [ ] `package.json`'da publisher: `tw4` belirtildi
- [ ] Extension package'landı (`vsce package`)
- [ ] Login başarılı (`vsce login tw4`)
- [ ] Publish komutu çalıştırıldı (`vsce publish`)

## İletişim

Sorun devam ederse:
1. Manual upload yöntemini kullan
2. Marketplace support'a başvur
3. GitHub Issues'da destek iste