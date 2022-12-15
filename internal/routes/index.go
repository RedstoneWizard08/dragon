package routes

import (
	"encoding/json"
	"io/fs"

	"github.com/RedstoneWizard08/dragon/internal/common"
	"github.com/gofiber/fiber/v2"
)

func RouteIndex(ctx *fiber.Ctx) error {
	info, err := fs.ReadFile(common.ViewsFS, "index.html.json")

	injectHead := ""
	injectBody := ""

	if err == nil {
		var descriptor common.ViewMetadata
		err := json.Unmarshal(info, &descriptor)

		if err == nil {
			injectHead, injectBody = RenderViewAssets(descriptor)
		}
	}

	return ctx.Render("index", fiber.Map{
		"Title":      "Home",
		"InjectBody": injectBody,
		"InjectHead": injectHead,
	})
}
