from flask import Flask, request, jsonify

from main import my_sum

app = Flask(__name__)


@app.route('/get_sum', methods=['GET', 'POST'])
def example_get():
    # curl "http://127.0.0.1:5000/get_sum?sum1=100&sum2=200"
    if request.method == 'GET':
        data1 = request.args.get('sum1')
        data2 = request.args.get('sum2')
        sum_result = my_sum(int(data1), int(data2))
        return jsonify({'data1': data1, 'data2': data2, 'sum': sum_result})


if __name__ == '__main__':
    app.run('0.0.0.0')
