package sass

func NewCompiler(options SassCompiler) SassCompiler {
	opts := options
	empty := SassCompiler{}
	defaults := GetDefaultOptions()

	if opts.SourceMap == empty.SourceMap {
		opts.SourceMap = defaults.SourceMap
	} else {
		sourceMap := opts.SourceMap
		sourceMapEmpty := empty.SourceMap
		sourceMapDefaults := defaults.SourceMap

		if sourceMap.EmbedSourceMapsInCSS == sourceMapEmpty.EmbedSourceMapsInCSS {
			sourceMap.EmbedSourceMapsInCSS = sourceMapDefaults.EmbedSourceMapsInCSS
		}

		if sourceMap.EmbedSourcesInMaps == sourceMapEmpty.EmbedSourcesInMaps {
			sourceMap.EmbedSourcesInMaps = sourceMapDefaults.EmbedSourcesInMaps
		}

		if sourceMap.Enable == sourceMapEmpty.Enable {
			sourceMap.Enable = sourceMapDefaults.Enable
		}

		if sourceMap.UrlLinkingType == sourceMapEmpty.UrlLinkingType {
			sourceMap.UrlLinkingType = sourceMapDefaults.UrlLinkingType
		}

		opts.SourceMap = sourceMap
	}

	if opts.OutputStyle == empty.OutputStyle {
		opts.OutputStyle = defaults.OutputStyle
	}

	if opts.EmitCharset == empty.EmitCharset {
		opts.OutputStyle = defaults.OutputStyle
	}

	if opts.EmitErrorCSS == empty.EmitErrorCSS {
		opts.OutputStyle = defaults.OutputStyle
	}

	if opts.UpdateMode == empty.UpdateMode {
		opts.OutputStyle = defaults.OutputStyle
	}

	return opts
}
