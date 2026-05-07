# 魔理沙书屋 - 爱丽丝书屋数据全文搜索web

> 警告：只在linux运行过，其他系统不确定是否能正常运行

## 使用教程

### 克隆代码

```sh
git clone https://git.inf.li/nekonamic/alicesw-web.git
cd alicesw-web
```

### 将数据库与索引放在目录下

数据接口如下
```
alicesw-web/
├─ content_index/
├─ novel_index/
├─ data.db
...
```

### 构建

> node版本使用24

```sh
npx vite build
```

### 运行

```sh
node build
```
