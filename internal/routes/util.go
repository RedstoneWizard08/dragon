package routes

import (
	"fmt"
	"path"
	"strings"

	"github.com/RedstoneWizard08/dragon/internal/common"
	"github.com/RedstoneWizard08/dragon/pkg/logger"
	"github.com/RedstoneWizard08/dragon/pkg/sass"
	"github.com/evanw/esbuild/pkg/api"
)

func RenderViewAssets(descriptor common.ViewMetadata) (string, string) {
	log := logger.New("COMPILE")

	injectBody := ""
	injectHead := ""

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
		fileName := path.Join(common.GetWorkingDirectory(), "views", style)
		newFilePath := path.Join(common.GetWorkingDirectory(), "views", newFileName)

		compiler := sass.NewCompiler(sass.GetDefaultOptions())

		cerr := compiler.Compile(map[string]string{
			fileName: newFilePath,
		})

		if cerr != nil {
			log.Warn("Could not compile " + style + ".")
			fmt.Println(cerr)
			continue
		}

		injectHead += fmt.Sprintf("<link href=\"/%s\" type=\"text/css\" rel=\"stylesheet\" />", newFileName)
	}

	return injectHead, injectBody
}
