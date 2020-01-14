package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/saracen/walker"
	"github.com/spf13/viper"
	"github.com/urfave/cli"
)

var (
	ignores []string = []string{"build", "bin", "out", "node_modules", ".idea", ".vscode", ".gradle"}
)

func poiAction(c *cli.Context) error {
	err := viper.ReadInConfig()
	if err != nil {
		return err
	}

	var conf config
	if err := viper.Unmarshal(&conf); err != nil {
		return err
	}

	projectPaths, err := getProjectFullPaths(&conf)
	if err != nil {
		return err
	}
	otherFullPaths, err := getOtherFullPaths(&conf)
	if err != nil {
		return err
	}

	for _, path := range append(projectPaths, otherFullPaths...) {
		fmt.Println(path)
	}

	return nil
}

func getProjectFullPaths(conf *config) ([]string, error) {
	paths := make([]string, 0, 1000)
	for _, root := range conf.Projects {
		rootPath, err := getFullPath(root)
		if err != nil {
			return nil, err
		}

		err = walker.Walk(rootPath, func(path string, info os.FileInfo) error {
			if info.IsDir() && info.Name() == ".git" {
				parent := filepath.Dir(path)
				paths = append(paths, parent)
				return filepath.SkipDir
			} else if info.IsDir() && isIgnore(info) {
				return filepath.SkipDir
			}

			return nil
		})

		if err != nil {
			return nil, err
		}
	}

	return paths, nil
}

func isIgnore(info os.FileInfo) bool {
	for _, ignore := range ignores {
		if info.Name() == ignore {
			return true
		}
	}
	return false
}

func getOtherFullPaths(conf *config) ([]string, error) {
	paths := make([]string, 0, len(conf.Others))
	for _, other := range conf.Others {
		fullPath, err := getFullPath(other)
		if err != nil {
			return nil, err
		}
		paths = append(paths, fullPath)
	}

	return paths, nil
}
