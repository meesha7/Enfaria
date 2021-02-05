use crate::prelude::*;

pub fn routes(app: &mut Server<State>) {
    app.at("release/windows").get(|_| async move {
        let file = std::fs::read("release/windows/enfaria-game.zip")?;
        let mut body = Body::from_bytes(file);
        body.set_mime("application/zip");
        Ok(Response::builder(200)
            .header("Content-Disposition", r#"attachment; filename="enfaria-game.zip""#)
            .body(body)
            .build())
    });

    app.at("release/linux").get(|_| async move {
        let file = std::fs::read("release/linux/enfaria-game.zip")?;
        let mut body = Body::from_bytes(file);
        body.set_mime("application/zip");
        Ok(Response::builder(200)
            .header("Content-Disposition", r#"attachment; filename="enfaria-game.zip""#)
            .body(body)
            .build())
    });
}
