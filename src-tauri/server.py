from flask import Flask, request, jsonify
from flask_cors import CORS
import subprocess
import tempfile

app = Flask(__name__)
CORS(app)  # 启用 CORS

@app.route('/run-code', methods=['POST'])
def run_code():
    data = request.json
    code = data.get('code', '')

    print('Received code:', code)

    with tempfile.NamedTemporaryFile(suffix='.py', delete=False) as temp_script:
        temp_script.write(code.encode('utf-8'))
        temp_script.flush()
        try:
            result = subprocess.run(['python', temp_script.name], capture_output=True, text=True, check=True)
            output = result.stdout
            error = result.stderr
            print('Execution output:', output)
            print('Execution error:', error)
        except subprocess.CalledProcessError as e:
            output = e.stdout
            error = e.stderr
            print('Execution error:', error)

    return jsonify({'output': output, 'error': error})

if __name__ == '__main__':
    app.run(debug=True)
