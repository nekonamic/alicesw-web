# 魔理沙书屋 - 可以在家里NAS运行支持搜索文章内容的爱丽丝书屋

预览网站: https://marisa.inf.li/

## Windows用户

可以直接下载构建好的程序：https://r2.inf.li/alicesw-web.7z

还要下载数据: https://r2.inf.li/alicesw-20260428.7z

数据也许隔三岔五会更新，当前数据截止2026/04/28

解压好后把数据放在alicesw-web文件夹里，如下

```
alicesw-web/
├─ content_index/
├─ novel_index/
├─ data.db
...其他东西
```

然后双击`start.bat`就能启动了

然后打开浏览器，访问 `http://127.0.0.1:3000`

### 开发环境

`rust`, `node`, `pnpm`

开发的时候都用的最新stable

## 使用教程

### 克隆web代码

```sh
git clone https://git.inf.li/nekonamic/alicesw-web.git
cd alicesw-web
```

### 把下载得到的数据库与索引放在现在的目录下

结构如下
```
alicesw-web/
├─ content_index/
├─ novel_index/
├─ data.db
...
```

### 构建

```sh
# 需要先编译rust
cd ./packages/fts-native
pnpm build
cd ../..

# 再构建SvelteKit
pnpm i
pnpm build
```

### 运行

```sh
node build
```

### 更新

```sh
# 在alicesw-web文件夹内
git pull
# 编译rust
cd ./packages/fts-native
pnpm build
cd ../..
# 构建SvelteKit
pnpm i
pnpm build
# 运行
node build
```
