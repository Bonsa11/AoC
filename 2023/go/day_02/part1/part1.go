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
	ID    int
	valid bool
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
	round = Round{red: 0, blue: 0, green: 0}
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
	games := make([]Game, len(data.Lines))
	maxRound := Round{red: 12, green: 13, blue: 14}
	for i, line := range data.Lines {
		splitLine := strings.Split(line, ":")
		game := Game{valid: true}
		game.ID, err = strconv.Atoi(strings.Split(splitLine[0], " ")[1])
		if err != nil {
			log.Fatalf("couldn;t get ID from %s", splitLine[0])
		}
		roundStrings := strings.Split(splitLine[1], ";")
		rounds := make([]Round, len(roundStrings))
		for idx, round := range roundStrings {
			newRound := stringToRound(round)
			rounds[idx] = newRound
			if newRound.red > maxRound.red || newRound.blue > maxRound.blue || newRound.green > maxRound.green {
				game.valid = false
			}
		}
		games[i] = game
	}
	sumOfIdValues = 0
	for _, game := range games {
		if game.valid {
			sumOfIdValues += game.ID
		}
	}

	return sumOfIdValues, nil
}
