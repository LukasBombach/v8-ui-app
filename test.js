// Print helper function, calling Deno.core.print()
function print(value) {
  Deno.core.print(value.toString()+"\n");
}
const arr = [1, 2, 3];
print("The sum of");
print(arr);
print("is");
print(Deno.core.opSync('op_sum', arr));
/* // And incorrect usage
try {
  print(Deno.core.opSync('op_sum', 0));
} catch(e) {
  print('Exception:');
  print(e);
} */