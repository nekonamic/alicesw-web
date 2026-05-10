# 魔理沙书屋 - 可以在家里NAS运行支持搜索文章内容的爱丽丝书屋

预览网站: https://marisa.inf.li/

网站只有IPv6，访问需要有IPv6地址，可以使用手机流量或者打开路由器的IPv6功能

数据下载: https://r2.inf.li/alicesw-20260428.7z

## 编译教程

### 开发环境

`rust`, `node`, `pnpm`

开发的时候都用的最新stable

### 克隆索引代码

```sh
git clone https://git.inf.li/nekonamic/alicesw_fts.git
cd alicesw_fts
```

### 索引数据

把下载得到的`.db`文件重命名为`data.db`，然后放在`alicesw_fts/`文件夹里，运行

```sh
cargo run --release
```

完成后会得到`content_index/` `novel_index/`以及老的`data.db`

### 克隆web代码

```sh
git clone https://git.inf.li/nekonamic/alicesw-web.git
cd alicesw-web
```

### 把刚刚得到的数据库与索引放在现在的目录下

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
