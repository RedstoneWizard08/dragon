package common

import (
	"html/template"
	"net/http"

	"github.com/RedstoneWizard08/dragon/pkg/config"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/template/html"
)

func GetFiberConfig() fiber.Config {
	engine := html.NewFileSystem(http.FS(ViewsFS), ".html")

	engine.AddFunc(
		"unescape", func(s string) template.HTML {
			return template.HTML(s)
		},
	)

	return fiber.Config{
		DisableStartupMessage: true,
		Views:                 engine,
	}
}

func GetAppConfig() config.Config {
	return config.Parse()
}
