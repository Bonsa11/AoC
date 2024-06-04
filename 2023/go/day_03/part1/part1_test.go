package main

import (
	"testing"
)

const TEST_ANSWER int = 4361

func TestRun(t *testing.T) {
	fileName := "./test.txt"
	data, err := parse(fileName)
	if err != nil {
		t.Fatalf("unable to read in %s", fileName)
	}

	val, err := run(*data)
	if err != nil {
		t.Fatalf("unable to parse data: %s", err)
	}

	if !(val == TEST_ANSWER) {
		t.Fatalf("Got wrong answer: %d expected %d", val, TEST_ANSWER)
	}
}
