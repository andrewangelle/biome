/* should not generate diagnostics */
const doSomethingTs = () => null;
try {
  doSomethingTs();
} catch(ex) {}

function fooTs(){
  try {
    doSomethingTs();
  } catch(ex) {}
}

const barTs = () => {
  try {
    doSomethingTs();
  } catch(ex) {}
}