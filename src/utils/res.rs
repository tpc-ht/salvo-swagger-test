use salvo::oapi::ToSchema;
use salvo::prelude::Json;

pub use crate::model::common_model::ResObj;

impl<T: ToSchema> ResObj<T> {
    pub fn ok(data: Option<T>) -> Self {
        Self {
            code: 200,
            msg: "调用成功".to_string(),
            data,
        }
    }
    pub fn custom(code: i32, err: String) -> Self {
        Self {
            code,
            msg: err,
            data: None,
        }
    }
    pub fn err(err: String) -> Self {
        Self {
            code: 100,
            msg: err,
            data: None,
        }
    }
    pub fn sys_err(err: String) -> Self {
        Self {
            code: 500,
            msg: err,
            data: None,
        }
    }
}

#[allow(dead_code)]
pub fn res_json_ok<T: ToSchema>(data: Option<T>) -> Json<ResObj<T>> {
    Json(ResObj::ok(data))
}
#[allow(dead_code)]
pub fn res_ok<T: ToSchema>(data: Option<T>) -> ResObj<T> {
    ResObj::ok(data)
}

#[allow(dead_code)]
pub fn res_json_custom<T: ToSchema>(code: i32, msg: String) -> Json<ResObj<T>> {
    Json(ResObj::custom(code, msg))
}
#[allow(dead_code)]
pub fn res_custom<T: ToSchema>(code: i32, msg: String) -> ResObj<T> {
    ResObj::custom(code, msg)
}
#[allow(dead_code)]
pub fn res_json_err<T: ToSchema>(msg: String) -> Json<ResObj<T>> {
    Json(ResObj::err(msg))
}
#[allow(dead_code)]
pub fn res_err<T: ToSchema>(msg: String) -> ResObj<T> {
    ResObj::err(msg)
}

#[allow(dead_code)]
pub fn res_sys_err<T: ToSchema>(msg: String) -> ResObj<T> {
    ResObj::sys_err(msg)
}

#[allow(dead_code)]
pub fn res_json_sys_err<T: ToSchema>(msg: String) -> Json<ResObj<T>> {
    Json(ResObj::sys_err(msg))
}

#[allow(dead_code)]
pub type Res<T> = Result<Json<ResObj<T>>, Json<ResObj<()>>>;

#[allow(dead_code)]
pub fn match_ok<T: ToSchema>(res: rbatis::Result<T>) -> Res<T> {
    match res {
        Ok(v) => Ok(res_json_ok(Some(v))),
        Err(err) => Err(res_json_sys_err(err.to_string())),
    }
}

#[allow(dead_code)]
pub fn match_custom_ok<T: ToSchema>(
    res: rbatis::Result<T>,
    resolve: Box<dyn FnOnce(T) -> Json<ResObj<T>>>,
) -> Res<T> {
    match res {
        Ok(v) => Ok(resolve(v)),
        Err(err) => Err(res_json_sys_err(err.to_string())),
    }
}

#[allow(dead_code)]
pub fn match_no_res_ok(res: rbatis::Result<bool>) -> Res<()> {
    match res {
        Ok(v) => {
            if v {
                Ok(res_json_ok(Some(())))
            } else {
                Err(res_json_sys_err("发生错误".to_string()))
            }
        }
        Err(err) => Err(res_json_sys_err(err.to_string())),
    }
}
