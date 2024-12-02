<?php

$s = file_get_contents("2example");
//$s = file_get_contents("2input");

$c = 0;
foreach (explode("\n", $s) as $l) {
	if ($l != "") {
		$x = explode(" ", $l);
		$el = $x[0];
		$inc = $desc = false;
		$good = true;
		array_shift($x);
		foreach ($x as $v) {
			if ($v - $el < -3 || $v - $el > 3 || $v == $el) {
				$good = false;
				break;
			} else if ($v - $el > 0) {
				if ($desc) { $good = false; break; }
				else { $inc = true; }
			} else if ($v - $el < 0) {
				if ($inc) { $good = false; break; }
				else { $desc = true; }
			}
			$el = $v;
		}
		if ($good) { $c++; }
	}
}

echo($c);
echo("<br/>");

function good($arr, $errnum, $inc, $desc) {
	if ($errnum > 1) { return false; }
	if ($inc && $desc) { return false; }
	if (sizeof($arr) == 1) { return true; }
	$el = $arr[0];
	array_shift($arr);
	$v = $arr[0];
	if ($v - $el < -3 || $v - $el > 3 || $v == $el) {
		$arr[0] = $el;
		return good($arr, $errnum + 1, $inc, $desc);
	} else if ($v - $el > 0) {
		if (good($arr, $errnum, true, $desc)) return true;
		else {
			$arr[0] = $el;
			return good($arr, $errnum + 1, $inc, $desc);
		}
	} else if ($v - $el < 0) {
		if (good($arr, $errnum, $inc, true)) return true;
		else {
			$arr[0] = $el;
			return good($arr, $errnum + 1, $inc, $desc);
		}
	}
}

$c = 0;
foreach (explode("\n", $s) as $l) {
	if ($l != "") {
		$x = explode(" ", $l);
		if (good($x, 0, false, false)) { $c++; }
		else {
			array_shift($x);
			if (good($x, 1, false, false)) { $c++; }
		}
	}
}

echo($c);
?>
