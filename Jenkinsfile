pipeline {
    agent {
        label 'rhel7'
    }
    environment {
        PATH = "$PATH:$HOME/.cargo/bin"

    }
    stages {
/*
        stage('rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are
                // required.
                sh "cargo fmt --all -- --check"
            }
        }
*/
        stage('build') {
            steps {
                sh "cargo build"
            }
        }
/*
        stage('test') {
            steps {
                sh "cargo test"
            }
        }
        stage('clippy') {
            steps {
                sh "cargo clippy --all"
            }
        }
        stage('tarpaulin') {
            steps {
                sh "cargo tarpaulin --out Xml"
                    cobertura coberturaReportFile: 'cobertura.xml'
            }
        }
        stage('rpm') {
            steps {
                sh "cargo rpm init"
                    sh "cargo rpm build"
            }
        }
*/
    }
}
