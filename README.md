# 魔理沙书屋 - 可以在家里NAS运行可以搜索文章内容的爱丽丝书屋

所谓NAS其实就是一台linux电脑，本地运行需要稍许软件开发知识，如果觉得麻烦可以直接使用预览网站: https://marisa.inf.li/

## 使用教程

数据下载地址: https://marisa.inf.li/

数据也许隔三岔五会更新，当前数据截止2026/04/28

先把下载的文件重命名成data.db

### 索引数据

```sh
# 这是索引数据的仓库
git clone https://git.inf.li/nekonamic/alicesw_fts
cd alicesw_fts
cargo run --release
```

然后会得到如下数据

```
alicesw_fts/
├─ content_index/
├─ novel_index/
├─ data.db
```

性能弱的电脑可能需要十几分钟

### 克隆web代码

```sh
git clone https://git.inf.li/nekonamic/alicesw-web.git
cd alicesw-web
```

### 将刚刚得到的数据库与索引放在现在的目录下

结构如下
```
alicesw-web/
├─ content_index/
├─ novel_index/
├─ data.db
...
```

### 构建

> node版本使用24，包管理使用pnpm

```sh
# 需要先编译rust
cd ./src-native
pnpm i
pnpm build
cd ..

# 再构建SvelteKit
pnpm i
pnpm build
```

### 运行

```sh
node build
```

默认3000端口
