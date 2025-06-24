pipeline {
  agent any

  parameters {
    string(name: 'MONGO_HOST', defaultValue: '', description: 'Mongo Dev Host IP')
    string(name: 'MONGO_PORT', defaultValue: '22', description: 'SSH Port')
    string(name: 'MONGO_USER', defaultValue: 'ppa', description: 'SSH Username')
    string(name: 'PROJECT_DIR', defaultValue: '~/axum_seaorm', description: 'Project directory path on server')
    string(name: 'REPO_URL', defaultValue: 'https://github.com/arisarisyi/axum_seaorm.git', description: 'GitHub Repo URL')
  }

  environment {
    SSH_CREDENTIAL_ID = credentials('mongo-dev') // ini dari Jenkins Credential
  }

  stages {
    stage('Deploy to Mongo Dev') {
      steps {
        sshagent([SSH_CREDENTIAL_ID]) {
          sh '''
          ssh -p $MONGO_PORT -o StrictHostKeyChecking=no $MONGO_USER@$MONGO_HOST <<'ENDSSH'
            source $HOME/.cargo/env

            if [ -d $PROJECT_DIR ]; then
              cd $PROJECT_DIR && git pull
            else
              git clone $REPO_URL $PROJECT_DIR && cd $PROJECT_DIR
            fi

            cd $PROJECT_DIR && cargo build --release

            pkill -f "target/release/axum_seaorm" || true
            nohup ./target/release/axum_seaorm > app.log 2>&1 &
          ENDSSH
          '''
        }
      }
    }
  }
}
