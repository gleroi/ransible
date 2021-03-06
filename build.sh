#!/usr/bin/env bash

set -o pipefail  # trace ERR through pipes
set -o errtrace  # trace ERR through 'time command' and other functions
set -o nounset   ## set -u : exit the script if you try to use an uninitialised variable
set -o errexit   ## set -e : exit the script if any statement returns a non-true return value

LIBRARY_DIR=library/

function setup {
	python3 -m venv venv
	activate
	pip install -r requirements.txt
}

function activate {
	source ./venv/bin/activate
}

function build {
	modules=$(find modules_rs -mindepth 1 -maxdepth 1 -type d)
	set +o errexit
	for module in $modules; do
		cargo install --bins --path $module --root $LIBRARY_DIR
	done
}

function playbook {
	ansible-playbook -i inventory.yml -v playbook.yml
}

$*
