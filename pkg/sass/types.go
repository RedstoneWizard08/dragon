package sass

type SourceMapURLLinker float32
type SassOutputStyle float32

const (
	RelativeLinking SourceMapURLLinker = 0.0 // Link source maps with relative paths.
	AbsoluteLinking SourceMapURLLinker = 0.1 // Link source maps with absolute paths.
)

const (
	ExpandedStyle   SassOutputStyle = 1.0 // Output expanded code.
	CompressedStyle SassOutputStyle = 1.1 // Output minifed/compressed code.
)

type SassWatchOptions struct {
	// Whether or not to enable watch mode.
	Enable bool

	// Use polling to check for changes when
	// watching, instead of a native watcher.
	UsePolling bool
}

type SassSourceMapOptions struct {
	// Whether or not to generate source maps.
	Enable bool

	// How to link source maps to source files.
	UrlLinkingType SourceMapURLLinker

	// Whether or not to embed source file
	// contents in source maps.
	EmbedSourcesInMaps bool

	// Whether or not to embed source maps
	// in CSS.
	EmbedSourceMapsInCSS bool
}

type SassCompiler struct {
	// Compiler watch options.
	Watch SassWatchOptions

	// Compiler source mapping options.
	SourceMap SassSourceMapOptions

	// Paths to resolve source files from.
	LoadPath string

	// The CSS output style.
	OutputStyle SassOutputStyle

	// Whether or not to emit an @charset or
	// BOM for CSS with non-ASCII characters.
	EmitCharset bool

	// When an error occurs, emit a stylesheet
	// describing them.
	EmitErrorCSS bool

	// Only compile out-of-date stylesheets.
	UpdateMode bool

	// Whether or not to stop once an error
	// occurs.
	StopOnError bool
}
