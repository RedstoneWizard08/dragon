package routes

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"strings"

	"github.com/evanw/esbuild/pkg/api"
	"github.com/RedstoneWizard08/dragon/pkg/go-libsass"

	"github.com/RedstoneWizard08/dragon/internal/common"
	"github.com/RedstoneWizard08/dragon/pkg/logger"
	"github.com/gofiber/fiber/v2"
)

func RouteIndex(ctx *fiber.Ctx) error {
	log := logger.New("COMPILE")
	info, err := fs.ReadFile(common.ViewsFS, "index.html.json")

	injectHead := ""
	injectBody := ""

	if err == nil {
		var descriptor common.ViewMetadata
		err := json.Unmarshal(info, &descriptor)

		if err == nil {
			for _, script := range descriptor.Scripts {
				newFileName := strings.ReplaceAll(script, "ts", "js")

				res := api.Build(api.BuildOptions{
					EntryPoints: []string{"views/" + script},
					Outfile:     "views/" + newFileName,
					Bundle:      true,
					Write:       true,
					LogLevel:    api.LogLevelSilent,
				})

				if len(res.Errors) > 0 {
					log.Warn(fmt.Sprintf("Could not compile " + script + "."))
					continue
				}

				injectBody += fmt.Sprintf("<script src=\"/%s\" type=\"module\"></script>", newFileName)
			}

			for _, style := range descriptor.Styles {
				newFileName := strings.ReplaceAll(style, "scss", "css")

				read, err := fs.ReadFile(common.ViewsFS, style)

				if err != nil {
					log.Warn("Could not read " + style + ".")
					continue
				}

				buf := bytes.NewBuffer(read)
				file, err := os.OpenFile("views/"+newFileName, os.O_CREATE|os.O_WRONLY, os.ModePerm)

				if err != nil {
					log.Warn(fmt.Sprintf("Could not open " + newFileName + "."))
					continue
				}

				file.Sync()

				transpiler, err := libsass.New(file, buf)

				if err != nil {
					log.Warn("Could not start transpiler.")
					file.Close()
					continue
				}

				err_ := transpiler.Run()

				if err_ != nil {
					log.Warn(fmt.Sprintf("Could not compile " + style + "."))
					file.Close()
					continue
				}

				file.Sync()
				file.Close()

				injectHead += fmt.Sprintf("<link href=\"/%s\" type=\"text/css\" rel=\"stylesheet\" />", newFileName)
			}
		}
	}

	return ctx.Render("index", fiber.Map{
		"Title":      "Home",
		"InjectBody": injectBody,
		"InjectHead": injectHead,
	})
}
