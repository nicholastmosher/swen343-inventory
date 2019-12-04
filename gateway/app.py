from flask import Flask, url_for, request, make_response, redirect
app = Flask(__name__)

def save_cookie(res, key, val):
    res.set_cookie(key, value=val, domain='.kennuware.xyz')

@app.route('/dev')
def dev():
    # redirect to the old page
    return redirect("http://kennuware.xyz:666/dev")

@app.route('/')
def api_root():
    try:
        # get query params
        token = request.args.get('authorization')
        email = request.args.get('email')
        url = request.args.get('redirect_url')
        print(f"Token ({token}), Email ({email}), URL ({url})")

        # redirect to requested page
        print(url)
        res = make_response(redirect(f"http://{url}"))
    
        # add details to cookies (session)
        print(f"adding details to browser cookies")
        save_cookie(res, "kuw-token", token)
        save_cookie(res, "kuw-email", email)
    except:
        print("Query params were not properly received")

        # assume none were given, redirect to old home page
        return redirect("http://kennuware.xyz:666/")

    return res

if __name__ == '__main__':
    app.run(host="kennuware.xyz", port=80)

