/* should not generate diagnostics */
function foo() {
  let a;
}

const bar = () => {
  let b;
}


function fooWithComment() {
  // should work
}

const barWithComment = () => {
  // should work
}

function fooWithMultilineComment() {
  /**
   * this should also work 
   */
}

const barWithMultilineComment = () => {
  /**
   * this should also work 
   */
}


if (foo) {
  // empty
}

while (foo) {
  /* empty */
}

try {
  doSomething();
} catch (ex) {
  // continue regardless of error
}

try {
  doSomething();
} finally {
  /* continue regardless of error */
}

class Foo {
  static {
      bar();
  }
}

class Foo {
  static {
      // comment
  }
}

// biome-ignore lint/nursery/noEmptyBlock: this should be allowed
function shouldNotFail() {
  
}