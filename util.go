package main

import (
	"os/user"
	"strings"
)

func getFullPath(path string) (string, error) {
	usr, err := user.Current()
	if err != nil {
		return "", err
	}
	newPath := strings.Replace(path, "~", usr.HomeDir, 1)
	return newPath, nil
}
