package config

import (
	"errors"
	"os"
	"strings"

	"github.com/RedstoneWizard08/dragon/pkg/logger"
)

type config_no_file struct {
	Host string `yaml:"host" json:"host" toml:"host" env:"DRAGON_SERVER_HOST"`
	Port int    `yaml:"port" json:"port" toml:"port" env:"DRAGON_SERVER_PORT"`
}

type Config struct {
	Host string
	Port int

	file string
}

func (config *Config) Save() {
	is_json := strings.HasSuffix(config.file, ".json")
	is_toml := strings.HasSuffix(config.file, ".toml")
	is_yaml := strings.HasSuffix(config.file, ".yaml") || strings.HasSuffix(config.file, ".yml")

	if is_json {
		config.SaveJSON()
	} else if is_toml {
		config.SaveTOML()
	} else if is_yaml {
		config.SaveYAML()
	} else if config.file == "__ENV__" {
		log := logger.New("SAVE")
		log.Warn("Config is based on environment - not saving.")
	}
}

func Parse() Config {
	json_file, json_err := os.Stat("Dragon.json")
	toml_file, toml_err := os.Stat("Dragon.toml")
	yaml_file, yaml_err := os.Stat("Dragon.yaml")
	yml_file, yml_err := os.Stat("Dragon.yml")

	json_exists := !(errors.Is(json_err, os.ErrNotExist) || json_file.IsDir())
	toml_exists := !(errors.Is(toml_err, os.ErrNotExist) || toml_file.IsDir())
	yaml_exists := !(errors.Is(yaml_err, os.ErrNotExist) || yaml_file.IsDir())
	yml_exists := !(errors.Is(yml_err, os.ErrNotExist) || yml_file.IsDir())

	if !json_exists && !toml_exists && !yaml_exists && !yml_exists {
		return return_default("No file found! Using default config.")
	}

	var config Config

	if json_exists {
		config = parse_json_config()
	} else if toml_exists {
		config = parse_toml_config()
	} else if yaml_exists {
		config = parse_yaml_config("Dragon.yaml")
	} else if yml_exists {
		config = parse_yaml_config("Dragon.yml")
	} else {
		config_tmp, exists := parse_env_config()

		if exists {
			config = config_tmp
		}
	}

	return config
}
