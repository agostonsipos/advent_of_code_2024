<?php

$s = file_get_contents("5example");
$s = file_get_contents("5input");

$a = explode("\n\n", $s);

$raw_rules = $a[0];
$raw_seqs = $a[1];

$rules = [];
foreach (explode("\n", $raw_rules) as $raw_rule) {
    $rules[] = explode("|", $raw_rule);
}

$seqs = [];
foreach (explode("\n", $raw_seqs) as $raw_seq) {
    $seqs[] = explode(",", $raw_seq);
}

$bad_seqs = [];
$s = 0;
foreach ($seqs as $seq) {
    $ok = true;
    for ($i = 0; $i < sizeof($seq); ++$i) {
        for ($j = $i+1; $j < sizeof($seq); ++$j) {
            foreach ($rules as $rule) {
                if ($rule[0] == $seq[$j] && $rule[1] == $seq[$i]) {
                    $ok = false;
                    goto done;
                }
            }
        }
    }
    done:
    if ($ok) {
        $s += $seq[intdiv(sizeof($seq), 2)];
    } else {
        $bad_seqs[] = $seq;
    }
}

echo($s."\n");
echo("<br/>\n");

$s = 0;
foreach ($bad_seqs as $seq) {
    $mutable_seq = $seq;
    $ok = true;
    for ($i = 0; $i < sizeof($seq); ++$i) {
        for ($j = $i+1; $j < sizeof($seq); ++$j) {
            foreach ($rules as $rule) {
                if ($rule[0] == $seq[$j] && $rule[1] == $seq[$i]) {
                    $seq[$i] = $rule[0];
                    $seq[$j] = $rule[1];
                }
            }
        }
    }
    $s += $seq[intdiv(sizeof($seq), 2)];
}

echo($s."\n");
