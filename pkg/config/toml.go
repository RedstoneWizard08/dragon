package config

import (
	"errors"
	"fmt"
	"os"

	"github.com/RedstoneWizard08/dragon/pkg/logger"
	"github.com/pelletier/go-toml/v2"
)

func parse_toml_config() Config {
	file_content, err := os.ReadFile("Dragon.toml")

	if err != nil {
		return return_default("Unable to read the configuration file!")
	}

	var config config_no_file

	err2 := toml.Unmarshal(file_content, &config)

	if err2 != nil {
		return return_default("Unable to parse the configuration file!")
	}

	return Config{
		Host: config.Host,
		Port: config.Port,
		file: "Dragon.toml",
	}
}

func (config *Config) SaveTOML() {
	file_name := config.file
	_t, err := os.Stat(config.file)

	if errors.Is(err, os.ErrNotExist) {
		_f, err := os.Create(config.file)

		check_err(err, "Unable to create a new file!")

		_f.Close()
	}

	_t, err2 := os.Stat(config.file)

	check_err(err2, "Unable to get file info!")

	if _t.IsDir() {
		panic(errors.New("Unable to write to a directory!"))
	}

	d, err := toml.Marshal(&config)

	check_err(err, "Unable to serialize the configuration!")

	file, err := os.OpenFile(file_name, os.O_CREATE|os.O_WRONLY|os.O_APPEND, os.ModePerm)

	check_err(err, "Unable to open the file for writing!")

	file.Sync()

	bytes, err := file.WriteString(string(d))

	check_err(err, "Unable to write to the file!")

	log := logger.New("SERIALIZE")
	log.Info(fmt.Sprintf("Wrote %d bytes!", bytes))

	file.Sync()
	file.Close()
}
