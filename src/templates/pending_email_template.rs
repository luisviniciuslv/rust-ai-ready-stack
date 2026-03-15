pub const PENDING_EMAIL_TEMPLATE: &str = r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html dir="ltr" lang="pt-BR">
  <head>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <meta name="x-apple-disable-message-reformatting" />
  </head>
  <body style="background-color:rgb(255,255,255)">
    <table border="0" width="100%" cellpadding="0" cellspacing="0" role="presentation" align="center">
      <tbody>
        <tr>
          <td style="background-color:rgb(255,255,255);font-family:HelveticaNeue,Helvetica,Arial,sans-serif">
            <table
              align="center"
              width="100%"
              border="0"
              cellpadding="0"
              cellspacing="0"
              role="presentation"
              style="max-width:360px;background-color:rgb(255,255,255);border-style:solid;border-width:1px;border-color:rgb(238,238,238);border-radius:0.25rem;box-shadow:0 4px 6px -1px rgba(20,50,70,.2),0 2px 4px -2px rgba(20,50,70,.2);margin:0 auto;padding-top:40px;padding-right:0;padding-left:0;padding-bottom:48px"
            >
              <tbody>
                <tr style="width:100%">
                  <td>
                    <p
                      style="font-size:11px;line-height:16px;color:rgb(10,133,234);font-weight:700;letter-spacing:0;margin-top:16px;margin-bottom:8px;margin-right:8px;margin-left:8px;text-transform:uppercase;text-align:center"
                    >
                      Aviso Automatico
                    </p>
                    <h1
                      style="color:rgb(0,0,0);font-weight:500;font-family:HelveticaNeue-Medium,Helvetica,Arial,sans-serif;display:block;font-size:20px;line-height:24px;margin-bottom:0;margin-top:0;text-align:center;padding:0 20px"
                    >
                      Nova pendencia atribuida a você
                    </h1>
                    <p
                      style="font-size:14px;line-height:22px;color:rgb(68,68,68);padding:0 30px;margin-top:16px;margin-bottom:0;text-align:center"
                    >
                      Você foi marcado em um ticket no chamado <b>__CASE_ID__</b>.
                    </p>
                    <p
                      style="font-size:14px;line-height:22px;color:rgb(68,68,68);padding:0 30px;margin-top:8px;margin-bottom:0;text-align:center"
                    >
                      <b>Autor:</b> __AUTHOR__
                    </p>
                    <p
                      style="font-size:14px;line-height:22px;color:rgb(68,68,68);padding:0 30px;margin-top:8px;margin-bottom:0;text-align:center"
                    >
                      <b>Mensagem:</b> __MESSAGE__
                    </p>
                    <table
                      align="center"
                      width="100%"
                      border="0"
                      cellpadding="0"
                      cellspacing="0"
                      role="presentation"
                      style="background-color:rgba(0,0,0,.05);border-radius:0.25rem;margin-right:auto;margin-left:auto;margin-top:16px;margin-bottom:14px;vertical-align:middle;width:280px"
                    >
                      <tbody>
                        <tr>
                          <td style="text-align:center;padding:16px 8px">
                            <a
                              href="__CASE_URL__"
                              style="display:inline-block;background-color:rgb(10,133,234);color:rgb(255,255,255);font-size:14px;font-weight:700;line-height:20px;padding:12px 18px;border-radius:6px;text-decoration:none"
                            >
                              Acessar chamado
                            </a>
                          </td>
                        </tr>
                      </tbody>
                    </table>
                  </td>
                </tr>
              </tbody>
            </table>
            <p
              style="font-size:12px;line-height:23px;color:rgb(0,0,0);font-weight:800;letter-spacing:0;margin:0;margin-top:20px;text-align:center;text-transform:uppercase"
            >
              Notificacao automatica Company Compass.
            </p>
          </td>
        </tr>
      </tbody>
    </table>
  </body>
</html>
"#;
