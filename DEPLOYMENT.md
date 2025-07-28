# QuickNote GitHub Pages 部署指南

## 🚀 自动部署设置

### 1. 启用 GitHub Pages

1. 进入您的 GitHub 仓库设置
2. 找到 "Pages" 选项
3. 在 "Source" 部分选择 "GitHub Actions"
4. 保存设置

### 2. 推送代码触发部署

```bash
git add .
git commit -m "Add GitHub Pages deployment configuration"
git push origin main
```

部署完成后，您的网站将可以通过以下地址访问：
`https://[您的用户名].github.io/QuickNote-for-macOS/`

## 🔍 Google Search Console 设置

### 1. 添加网站

1. 访问 [Google Search Console](https://search.google.com/search-console)
2. 点击 "添加资源"
3. 输入您的网站 URL：`https://[您的用户名].github.io/QuickNote-for-macOS/`
4. 选择 "HTML 标签" 验证方式

### 2. 验证网站所有权

1. 复制 Google 提供的验证代码
2. 在 `index.html` 文件中找到以下注释行：
   ```html
   <!-- <meta name="google-site-verification" content="YOUR_VERIFICATION_CODE" /> -->
   ```
3. 将 `YOUR_VERIFICATION_CODE` 替换为实际的验证代码
4. 取消注释，使其变为：
   ```html
   <meta name="google-site-verification" content="您的实际验证代码" />
   ```
5. 提交并推送更改：
   ```bash
   git add .
   git commit -m "Add Google Search Console verification"
   git push origin main
   ```

### 3. 提交 Sitemap

1. 在 Google Search Console 中，进入 "Sitemaps" 部分
2. 添加您的 sitemap URL：`https://[您的用户名].github.io/QuickNote-for-macOS/sitemap.xml`
3. 提交 sitemap

## 📊 监控和分析

### Google Search Console 功能

- **性能报告**：查看搜索查询、点击次数、展示次数等
- **索引覆盖范围**：监控页面索引状态
- **移动设备可用性**：检查移动端兼容性
- **核心网页指标**：监控页面加载性能
- **增强功能**：设置结构化数据

### 建议的监控指标

1. **搜索查询**：了解用户如何找到您的网站
2. **点击率**：优化标题和描述
3. **平均排名**：跟踪关键词排名变化
4. **索引状态**：确保重要页面被索引

## 🔧 本地测试

在推送之前，您可以在本地测试网站版本：

```bash
# 构建网站版本
npm run build:web

# 预览构建结果
npm run preview
```

## 📝 注意事项

1. **Base URL**：确保所有资源路径都使用相对路径
2. **404 页面**：考虑添加自定义 404 页面
3. **性能优化**：定期检查页面加载速度
4. **内容更新**：保持网站内容与桌面应用同步

## 🆘 故障排除

### 常见问题

1. **部署失败**：检查 GitHub Actions 日志
2. **页面无法访问**：确认 GitHub Pages 已启用
3. **验证失败**：确保验证代码正确添加到 HTML 中
4. **Sitemap 错误**：检查 sitemap.xml 格式是否正确

### 获取帮助

如果遇到问题，请检查：
- GitHub Actions 运行日志
- Google Search Console 错误报告
- 浏览器开发者工具控制台 