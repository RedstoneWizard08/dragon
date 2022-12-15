package sass

func GetDefaultOptions() SassCompiler {
	return SassCompiler{
		SourceMap: SassSourceMapOptions{
			Enable:               true,
			UrlLinkingType:       RelativeLinking,
			EmbedSourcesInMaps:   false,
			EmbedSourceMapsInCSS: false,
		},

		OutputStyle:  ExpandedStyle,
		EmitCharset:  true,
		EmitErrorCSS: true,
		UpdateMode:   true,
	}
}

func GetDefaultArguments() string {
	return "--no-color --no-unicode --quiet --quiet-deps --no-stdin "
}
