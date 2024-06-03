package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Data struct {
	Lines []string
}

type Round struct {
	red   int
	blue  int
	green int
}

type Game struct {
	ID int
}

func main() {

	fileName := "../input.txt"

	log.SetFlags(log.Ldate)

	data, err := parse(fileName)
	if err != nil {
		log.Fatal(err)
	}

	sumOfIdValues, err := run(*data)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(sumOfIdValues)
}

func parse(fileName string) (data *Data, err error) {
	// read in file from os into buffer
	buf, err := os.ReadFile(fileName)
	if err != nil {
		return nil, fmt.Errorf("read input file: %w", err)
	}

	d := &Data{}
	lines := strings.Split(string(buf), "\n")
	for _, line := range lines {
		if line == "" {
			continue
		}
		d.Lines = append(d.Lines, line)
	}
	return d, nil
}

func stringToRound(roundString string) (round Round) {
	values := strings.Split(roundString, ",")
	round = Round{}
	for _, v := range values {
		valueString := strings.Split(strings.TrimSpace(v), " ")
		number := valueString[0]
		count, err := strconv.Atoi(strings.TrimSpace(number))
		if err != nil {
			log.Fatalf("unable to change %s to number", number)
		}
		colour := strings.TrimSpace(valueString[1])
		if colour == "blue" {
			round.blue = count
		} else if colour == "red" {
			round.red = count
		} else {
			round.green = count
		}
	}

	return round
}

func run(data Data) (sumOfIdValues int, err error) {
	sumOfIdValues = 0
	for _, line := range data.Lines {
		minRound := Round{red: 0, green: 0, blue: 0}
		splitLine := strings.Split(line, ":")
		roundStrings := strings.Split(splitLine[1], ";")
		for _, round := range roundStrings {
			newRound := stringToRound(round)
			if newRound.red > minRound.red {
				minRound.red = newRound.red
			}
			if newRound.blue > minRound.blue {
				minRound.blue = newRound.blue
			}
			if newRound.green > minRound.green {
				minRound.green = newRound.green
			}
		}
		val := minRound.red * minRound.blue * minRound.green
		log.Println(minRound)
		sumOfIdValues += val
	}
	return sumOfIdValues, nil
}
