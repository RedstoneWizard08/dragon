package config

import (
	"github.com/RedstoneWizard08/dragon/pkg/logger"
)

func check_err(err error, msg string) {
	if err != nil {
		log := logger.New("FATAL")
		log.Error(msg)

		panic(err)
	}
}

func get_default() Config {
	return Config{
		Host: "127.0.0.1",
		Port: 4000,
		file: "Dragon.toml",
	}
}

func get_default_no_file() config_no_file {
	return config_no_file{
		Host: "127.0.0.1",
		Port: 4000,
	}
}

func return_default(msg string) Config {
	log := logger.New("PARSE")
	log.Info(msg)

	return get_default()
}

func ite[T string | bool | int](condition bool, a T, b T) T {
	if condition {
		return a
	}

	return b
}
