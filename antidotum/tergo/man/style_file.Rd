% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/styling.R
\name{style_file}
\alias{style_file}
\title{Style a file}
\usage{
style_file(file, configuration = list())
}
\arguments{
\item{file}{(\code{character}) the file to format}

\item{configuration}{(\code{list}) Configuration for formatting. Default \code{list()}.}
}
\value{
No return value, called for side effects.
}
\description{
Style a file
}
\details{
To see possible configuration options, see \code{\link[=get_default_config]{get_default_config()}}.
}
\examples{
tmp <- tempfile()
file_conn <- file(tmp)
writeLines(c("function(){}", "A<-7"), file_conn)
close(file_conn)
style_file(file = tmp, configuration = list())
unlink(tmp)
}
