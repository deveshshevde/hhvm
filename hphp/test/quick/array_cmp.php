<?hh
<<__EntryPoint>> function main(): void {
  $x = array('a' => 1);
  $y = array('b' => 2);
  var_dump($x <= $y);
  var_dump($x > $y);
  echo "\n";
  var_dump($x < $y);
  var_dump($x >= $y);
  echo "\n";
  var_dump($x == $y);
  var_dump($x != $y);
  echo "\n";
  var_dump($x === $y);
  var_dump($x !== $y);
  echo "\n";
}
