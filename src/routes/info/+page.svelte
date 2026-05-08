<div class="py-6 px-2 sm:py-12">
  <article class="mx-auto max-w-3xl p-6 sm:p-12 prose">
    <h1>使用说明</h1>
    <div class="divider"></div>
    <h2>查询解析器语法 (Query Parser Syntax)</h2>
    <p>
      这个网站里的搜索栏有一些特殊的技巧，实际上Google等搜索引擎也支持其中部分语法
    </p>
    <p>
      别直接把想搜的东西直接输进去，假设你想搜<code>初音未来</code
      >，如果你在搜索框里直接输入<code>初音未来</code>，会被分词器分割成<code
        >初音</code
      >与<code>未来</code>，导致搜索出的结果是包含<code>初音</code>或<code
        >未来</code
      >的内容，如果只输入一个词比如<code>初音</code
      >，分词器无法再细分，得到的结果就会是包括<code>初音</code>的结果
    </p>
    <h3>精确短语 ""</h3>
    <p>
      如果你想准确的搜索<code>初音未来</code>不想被分词器分开，需要使用<code
        >"</code
      >把关键词括起来，比如<code>"初音未来"</code>
    </p>
    <h3>布尔运算符 + || -</h3>
    <ul>
      <li>
        <code>初音 + 未来</code>可以查询到同时有<code>初音</code>和<code
          >未来</code
        >的内容
      </li>
      <li>
        <code>初音 || 未来</code>可以查询到有<code>初音</code>或者有<code
          >未来</code
        >的内容，但中间留个空格，查询<code>初音 未来</code>效果是一样的
      </li>
      <li>
        <code>初音 -未来</code>可以查询到有<code>初音</code>但没有<code
          >未来</code
        >的内容
      </li>
    </ul>
    <p>还有一些别的语法，但中文搜索用不上，知道这些就够了</p>
  </article>
</div>
