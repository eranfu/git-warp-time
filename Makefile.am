ACLOCAL_AMFLAGS = -I build-aux
AM_DISTCHECK_CONFIGURE_FLAGS = --enable-developer-mode

.ONESHELL:
.SECONDARY:
.SECONDEXPANSION:
.DELETE_ON_ERROR:

docdir = $(datarootdir)/doc/$(TRANSFORMED_PACKAGE_NAME)
licensedir = $(datarootdir)/licenses/$(TRANSFORMED_PACKAGE_NAME)

bin_PROGRAMS = git-warp-time
git_warp_time_SOURCES = src/bin/git-warp-time.rs src/lib.rs src/cli.rs
EXTRA_git_warp_time_SOURCES =
EXTRA_DIST = Dockerfile hooks/build
dist_doc_DATA = README.md CHANGELOG.md
dist_license_DATA = LICENSE.md
nodist_man_MANS =

BUILT_SOURCES =

CLEANFILES =

DISTCLEANFILES = @AMINCLUDE@

# A classical use of the autoconf-archive include macro would expand
# INC_AMINCLUDE here, but the perl script that inlines include statements
# runs before the automake that organizes logic and performs substitution.
# Consequentially with a substitution here it becomes impossible to use
# automake conditionals and substitutions in the included Makefile fragments.
# By entering the expanded value directly we are ready in time for the inlining
# functionality and hence can use conditionals in included makefile fragments.
include $(top_srcdir)/aminclude.am

PHONY_DEVELOPER_TARGETS = release-preview
.PHONY: $(PHONY_DEVELOPER_TARGETS)

SEMVER := $(shell $(AWK) -F\" '/^version/ { print $$2 }' $(top_srcdir)/Cargo.toml)
TAG := v$(SEMVER)

if DEVELOPER_MODE

release-preview:
	$(GIT_CLIFF) --unreleased

CHANGELOG.md:
	$(GIT_CLIFF) -p $@ --unreleased -t $(TAG)

git-warp-time-%.md: CHANGELOG.md
	$(SED) -e '/^## \[$*\]/,/^## \[/!d' $< | \
		$(SED) -e '1s/^/Release v$*\n/' | \
		$(SED) -e '/^#/d' -e '/^$$/d' | \
		$(SED) -e 's/\*//g' | \
		$(SED) -e '1a\\' > $@

else !DEVELOPER_MODE

$(PHONY_DEVELOPER_TARGETS):
	@: $(error "Please reconfigure using --enable-developer-mode to use developer tooling")

endif !DEVELOPER_MODE
