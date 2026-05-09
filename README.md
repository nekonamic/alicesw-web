# 魔理沙书屋 - 可以在家里NAS运行支持搜索文章内容的爱丽丝书屋

所谓NAS其实就是一台linux电脑，本地运行需要稍许软件开发知识，如果觉得麻烦可以直接使用预览网站: https://marisa.inf.li/

## 开发环境

`rust`, `node`, `pnpm`

开发的时候都用的最新stable

## 使用教程

数据下载地址: https://r2.inf.li/alicesw-20260428.7z

数据也许隔三岔五会更新，当前数据截止2026/04/28

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
