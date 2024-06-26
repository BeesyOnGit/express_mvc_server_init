pub fn controllers_content(name: &str, crud: &bool) -> String {
    if !crud {
        let non_crud_file = format!(
            "import {{ Response, Request }} from 'express';
        import {{  editModelWithSave }} from '../Middleware/ServerFunctions';
        import {}Model from '../Models/{}Model';

        export const create{} = async (req: Request, res: Response) => {{
            const {{ body }} = req;
            try {{
                const new{} = await {}Model.create(body);
        
                if (!new{}) {{
                    return res.json({{ code: '011' }});
                }}
                const {{ _id }} = new{};
        
                return res.json({{ code: '' }});
            }} catch (error) {{
                console.log('🚀 ~ file: {}Controllers.ts:21 ~ create{} ~ error:', error);
            }}
        }};",
            name, name, name, name, name, name, name, name, name
        );
        return non_crud_file;
    }

    let crud_file = format!(
        "import {{ Response, Request }} from 'express';
    import {{  editModelWithSave }} from '../Middleware/ServerFunctions';
    import {}Model from '../Models/{}Model';
    
    export const create{} = async (req: Request, res: Response) => {{
        const {{ body }} = req;
        try {{
            const new{} = await {}Model.create(body);
    
            if (!new{}) {{
                return res.json({{ code: '011' }});
            }}
            const {{ _id }} = new{};
    
            return res.json({{ code: '' }});
        }} catch (error) {{
            console.log('🚀 ~ file: {}Controllers.ts:21 ~ create{} ~ error:', error);
        }}
    }};
    
    export const edit{} = async (req: Request, res: Response) => {{
        const {{ body, params, path }} = req;
        const {{ id }} = params;
        const filter = {{ _id: id }};
        try {{
            const find{} = await {}Model.findOne(filter);
    
            if (!find{}) {{
                return res.json({{ code: '010' }});
            }}
    
            editModelWithSave(find{}, body);
    
            const edited{} = await find{}.save();
    
            if (!edited{}) {{
                return res.json({{ code: '' }});
            }}
    
            return res.json({{ code: '' }});
        }} catch (error) {{
            console.log('🚀 ~ file: {}Controllers.ts:45 ~ edit{} ~ error:', error);
        }}
    }};
    export const delete{} = async (req: Request, res: Response) => {{
        const {{ params }} = req;
        const {{ id }} = params;
        const filter = {{ _id: id }};
        try {{
            const find{} = await {}Model.findOne(filter);
    
            if (!find{}) {{
                return res.json({{ code: '' }});
            }}
    
            const deleted{} = await find{}.delete();
    
            if (!deleted{}) {{
                return res.json({{ code: '' }});
            }}
    
            return res.json({{ code: '' }});
        }} catch (error) {{
            console.log('🚀 ~ file: {}Controllers.ts:67 ~ delete{} ~ error:', error);
        }}
    }};
    
    export const get{}s = async (req: Request, res: Response) => {{
        try {{
            const {}s = await {}Model.find();
    
            if (!{}s || {}s.length == 0) {{
                return res.json({{ code: '' }});
            }}
    
            return res.json({{ code: '', data: {}s }});
        }} catch (error) {{
            console.log('🚀 ~ file: {}Controllers.ts:88 ~ get{}s ~ error:', error);
        }}
    }};
    ",
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name
    );
    return crud_file;
}

pub fn model_content(name: &str) -> String {
    return format!(
        "import mongoose from 'mongoose'
    const {}Schema = new mongoose.Schema<{}Type>({{
        createdAt: {{
            type: Number,
            default: () => {{
                return Date.now();
            }},
        }},
    }});
    
    const {}Model: mongoose.Model<{}Type> = mongoose.model<{}Type>('{}', {}Schema);
    export default {}Model;
    
    // edit yout type here
    export type {}Type = {{
        createdAt?: number;
        _id?: string | mongoose.Schema.Types.ObjectId;
    }};
    ",
        name, name, name, name, name, name, name, name, name
    );
}

pub fn router_content(name: &str, crud: &bool) -> String {
    if !crud {
        return format!(
            "import express from 'express';
        // import your middlewars here
        // import {{ AuthVerification }} from '../Middleware/ServerFunctions';
        import {{ create{}}} from '../Controllers/{}Controllers';
        const {}Routes = express.Router();
        
        {}Routes.post('/', create{});
        
        export default {}Routes;
        ",
            name, name, name, name, name, name,
        );
    }

    return format!(
        "import express from 'express';
    // import your middlewars here
    // import {{ AuthVerification }} from '../Middleware/ServerFunctions';
    import {{ create{}, delete{}, edit{}, get{}s }} from '../Controllers/{}Controllers';
    const {}Routes = express.Router();
    
    {}Routes.post('/', create{});
    {}Routes.post('/edit/:id', edit{});
    {}Routes.delete('/:id', delete{});
    {}Routes.get('/', get{}s);
    
    export default {}Routes;
    ",
        name, name, name, name, name, name, name, name, name, name, name, name, name, name, name,
    );
}

pub fn app_content() -> String {
    return "import bodyParser from 'body-parser';
    import express, { Request, Response, NextFunction } from 'express';
    import http from 'http';
    import dotenv from 'dotenv';
    import compression from 'compression';
    import cors from 'cors';
    import { DbConnection } from './DBConnection';
    //imports for your modules (auto generated) //split ^

    dotenv.config();
    
    //Constatnts definition
    const app: any = express();
    const PORT = process.env.PORT;
    http.createServer(app);
    
    //Variable deffinition
    
    //Db connection//
    
    DbConnection('YOUR_DB_NAME_HERE', 'mongodb://127.0.0.1:27017');

    //Midelware
    app.use((req: Request, res: Response, next: NextFunction) => {
        res.header('Access-Control-Allow-Credentials', 'true');
        next();
    });
    app.use(
        compression({
            level: 6,
        })
    );
    
    //App upload limite setting
    app.use(bodyParser.json({ limit: '20mb' }));
    app.use(bodyParser.urlencoded({ limit: '50mb', extended: true }));
    
    //Define the authorized origins to communicate with
    app.use(bodyParser.json());
    app.use(cors({ origin: '*' }));
    
    //************************************ # API ROUTES (DO NOT DELETE) # ****************************************//
    //routing for your modules (auto generated) //split ^
    //************************************ # SERVER PORT SET # ****************************************//
    
    app.listen(PORT, () => {
        console.log(`Server live on port ${PORT}`);
    });
    ".to_string();
}

pub fn db_connect_content() -> String {
    return "import mongoose from 'mongoose';

    export const DbConnection = async (dbName: string, dbUrl: string) => {
        try {
            mongoose.set('strictQuery', false);
            const connection = await mongoose.connect(`${dbUrl}/${dbName}`);
            if (!connection) {
                return console.log('db connection unknown error');
            }
            console.log(`${dbName} database connected`);
        } catch (error) {
            console.error(error);
        }
    };
    "
    .to_string();
}

pub fn middleware_content() -> String {
    return "import { IncomingHttpHeaders } from 'http';
    import jwt from 'jsonwebtoken';
    import dotenv from 'dotenv';
    
    dotenv.config();
    
    const env = process.env.ENVIRONEMENT;
    const SITE_URL = process.env.SITE_URL;
    export type Headers = IncomingHttpHeaders & {
        isAdmin?: boolean;
        verifiedID?: string;
        userType?: number;
        userLocation?: string;
        userPass?: string;
        authorizationtoken?: string;
    };
    
    export const TokenVerifier = (token: string) => {
        try {
            if (token == undefined) {
                return 'no token';
            }
            return jwt.verify(token, process.env.TOKEN_ENCRIPTION_KEY!);
        } catch (error) {
        console.log('🚀 ~ file: middle.ts:25 ~ TokenVerifier ~ error:', error)
        }
    };
    
    export const generateToken = (id: string) => {
        try {
            if (!id) {
                return 'id and password are Mendatory';
            }
            return jwt.sign({ id }, process.env.TOKEN_ENCRIPTION_KEY!);
        } catch (error) {
        console.log('🚀 ~ file: middle.ts:36 ~ generateToken ~ error:', error)
        }
    };
    
    export const urlWitoutParams = (url: string) => {
        const urlArr = url.split('/');
        if (urlArr.length <= 4) {
            return urlArr.join('/');
        }
        urlArr.pop();
    
        return urlArr.join('/');
    };
    
    export const editModelWithSave = (model: any, edit: any) => {
        for (const key in edit) {
            model[key] = edit[key];
        }
        return model;
    };
    
    export const randomIdGenerator = (length: number) => {
        let characters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456789';
        let id = '';
        for (let i = 0; i < length; i++) {
            id += characters.charAt(Math.floor(Math.random() * characters.length));
        }
    
        return id;
    };
    "
    .to_string();
}

pub fn package_json_content() -> String {
    return r#"{
        "name": "api",
        "private": true,
        "version": "0.1.0",
        "main": "App.js",
        "scripts": {
            "build": "rimraf dist && npx tsc",
            "serv": "ts-node App/App.ts",
            "autogit": "cd App && git add . && git commit -F Com.txt && git push"
        },
        "dependencies": {
            "axios": "^1.6.8",
            "body-parser": "^1.20.0",
            "compression": "^1.7.4",
            "cors": "^2.8.5",
            "crypto-js": "^4.1.1",
            "dotenv": "^16.0.3",
            "express": "^4.18.1",
            "jsonwebtoken": "^9.0.2",
            "mongodb": "^4.11.0",
            "mongoose": "^6.6.1",
            "nodemon": "^3.1.0",
            "rimraf": "^4.3.1",
            "ts-node": "^10.9.1"
        },
        "devDependencies": {
            "@types/compression": "^1.7.2",
            "@types/cors": "^2.8.13",
            "@types/crypto-js": "^4.1.1",
            "@types/express": "^4.17.17",
            "@types/jsonwebtoken": "^9.0.1",
            "@types/mongoose": "^5.11.97",
            "@types/node": "^18.14.6"
        }
    }
    "#
    .to_string();
}

pub fn env_content() -> String {
    return r#"PORT= 3000
    TOKEN_ENCRIPTION_KEY= "your_token_encription_key_here"
    PASSWORD_ENCRIPTION_KEY= "your_password_encription_key_here""#
        .to_string();
}
