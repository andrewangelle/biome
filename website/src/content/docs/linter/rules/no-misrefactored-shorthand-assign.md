---
title: noMisrefactoredShorthandAssign (since vnext)
---

**Diagnostic Category: `lint/nursery/noMisrefactoredShorthandAssign`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow shorthand assign when variable appears on both sides.

This rule helps to avoid potential bugs related to incorrect assignments or unintended
side effects that may occur during refactoring.

Source: https://rust-lang.github.io/rust-clippy/master/#/misrefactored_assign_op

## Examples

### Invalid

```jsx
a += a + b
```

<pre class="language-text"><code class="language-text">nursery/noMisrefactoredShorthandAssign.js:1:1 <a href="https://biomejs.dev/lint/rules/no-misrefactored-shorthand-assign">lint/nursery/noMisrefactoredShorthandAssign</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Variable appears on both sides of an assignment operation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a += a + b
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This assignment might be the result of a wrong refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>a += b</strong></span><span style="color: lightgreen;"> instead.</span>
  
<strong>  </strong><strong>  1 │ </strong>a<span style="opacity: 0.8;">·</span>+=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">+</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>b
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span> 
</code></pre>

```jsx
a -= a - b
```

<pre class="language-text"><code class="language-text">nursery/noMisrefactoredShorthandAssign.js:1:1 <a href="https://biomejs.dev/lint/rules/no-misrefactored-shorthand-assign">lint/nursery/noMisrefactoredShorthandAssign</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Variable appears on both sides of an assignment operation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a -= a - b
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This assignment might be the result of a wrong refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>a -= b</strong></span><span style="color: lightgreen;"> instead.</span>
  
<strong>  </strong><strong>  1 │ </strong>a<span style="opacity: 0.8;">·</span>-=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">-</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>b
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span> 
</code></pre>

```jsx
a *= a * b
```

<pre class="language-text"><code class="language-text">nursery/noMisrefactoredShorthandAssign.js:1:1 <a href="https://biomejs.dev/lint/rules/no-misrefactored-shorthand-assign">lint/nursery/noMisrefactoredShorthandAssign</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Variable appears on both sides of an assignment operation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a *= a * b
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This assignment might be the result of a wrong refactoring.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>a *= b</strong></span><span style="color: lightgreen;"> instead.</span>
  
<strong>  </strong><strong>  1 │ </strong>a<span style="opacity: 0.8;">·</span>*=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">*</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>b
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span> 
</code></pre>

## Valid

```jsx
a += b
```

```jsx
a = a + b
```

```jsx
a = a - b
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
