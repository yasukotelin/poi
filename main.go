package main

import (
	"log"
	"os"

	"github.com/spf13/viper"
	"github.com/urfave/cli"
)

func main() {
	app := cli.NewApp()
	app.Name = "poi"
	app.Version = "1.0.0"
	app.Description = "poi is a Quick access command to project directories."
	app.Action = poiAction

	initConfig()

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}

func initConfig() {
	viper.SetConfigType("yaml")
	viper.SetConfigName("poi.yml")
	viper.AddConfigPath("$HOME/.config/poi")
}
