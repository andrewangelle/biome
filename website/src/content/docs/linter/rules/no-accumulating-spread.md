---
title: noAccumulatingSpread (since v1.0.0)
---

**Diagnostic Category: `lint/nursery/noAccumulatingSpread`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow the use of spread (`...`) syntax on accumulators.

Spread syntax allows an iterable to be expanded into its individual elements.

Spread syntax should be avoided on accumulators (like those in `.reduce`)
because it causes a time complexity of `O(n^2)` instead of `O(n)`.

Source: https://prateeksurana.me/blog/why-using-object-spread-with-reduce-bad-idea/

## Examples

### Invalid

```jsx
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => [...acc, val], []);
```

<pre class="language-text"><code class="language-text">nursery/noAccumulatingSpread.js:2:25 <a href="https://biomejs.dev/linter/rules/no-accumulating-spread">lint/nursery/noAccumulatingSpread</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid the use of spread (`...`) syntax on accumulators.</span>
  
    <strong>1 │ </strong>var a = ['a', 'b', 'c'];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a.reduce((acc, val) =&gt; [...acc, val], []);
   <strong>   │ </strong>                        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Spread syntax should be avoided on accumulators (like those in `.reduce`) because it causes a time complexity of `O(n^2)`.</span>
  
</code></pre>

```jsx
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => {return [...acc, val];}, []);
```

<pre class="language-text"><code class="language-text">nursery/noAccumulatingSpread.js:2:33 <a href="https://biomejs.dev/linter/rules/no-accumulating-spread">lint/nursery/noAccumulatingSpread</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid the use of spread (`...`) syntax on accumulators.</span>
  
    <strong>1 │ </strong>var a = ['a', 'b', 'c'];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a.reduce((acc, val) =&gt; {return [...acc, val];}, []);
   <strong>   │ </strong>                                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Spread syntax should be avoided on accumulators (like those in `.reduce`) because it causes a time complexity of `O(n^2)`.</span>
  
</code></pre>

```jsx
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => ({...acc, [val]: val}), {});
```

<pre class="language-text"><code class="language-text">nursery/noAccumulatingSpread.js:2:26 <a href="https://biomejs.dev/linter/rules/no-accumulating-spread">lint/nursery/noAccumulatingSpread</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid the use of spread (`...`) syntax on accumulators.</span>
  
    <strong>1 │ </strong>var a = ['a', 'b', 'c'];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a.reduce((acc, val) =&gt; ({...acc, [val]: val}), {});
   <strong>   │ </strong>                         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Spread syntax should be avoided on accumulators (like those in `.reduce`) because it causes a time complexity of `O(n^2)`.</span>
  
</code></pre>

## Valid

```jsx
var a = ['a', 'b', 'c'];
a.reduce((acc, val) => {acc.push(val); return acc}, []);
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
