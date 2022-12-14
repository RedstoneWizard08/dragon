package config

import (
	"os"
	"reflect"
	"strconv"
)

func get_env(env_var string, default_value string) (string, bool) {
	if val := os.Getenv(env_var); val != "" {
		return val, true
	} else {
		return default_value, false
	}
}

func parse_env_config() (Config, bool) {
	default_config := get_default_no_file()
	config_type := reflect.TypeOf(default_config)

	Host := default_config.Host
	Port := default_config.Port

	has_host := false
	has_port := false

	for i := 0; i < config_type.NumField(); i++ {
		field := config_type.Field(i)
		if env_var, ok := field.Tag.Lookup("env"); ok {
			if env_var == "" {
				continue
			} else {
				field_name := field.Name

				switch field_name {
				case "Host":
					val, has_var := get_env(env_var, Host)

					if has_var {
						has_host = true
					} else {
						has_host = false
					}

					Host = val

					break
				case "Port":
					env, has_var := get_env(env_var, strconv.Itoa(Port))

					if has_var {
						has_port = true
					} else {
						has_port = false
					}

					conv, err := strconv.Atoi(env)

					if err != nil {
						conv = Port
					}

					Port = conv

					break
				}
			}
		}
	}

	if has_host && has_port {
		return Config{
			Host: Host,
			Port: Port,
			file: "__ENV__",
		}, true
	} else {
		return Config{
			Host: Host,
			Port: Port,
			file: "__ENV__",
		}, false
	}
}
