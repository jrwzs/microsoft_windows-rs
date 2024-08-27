use windows::{core::*, Foundation::*, Win32::Foundation::*};

#[test]
fn ok() -> Result<()> {
    let a = IAsyncOperation::ready(Ok(123));
    let _: IAsyncInfo = a.cast()?;
    assert_eq!(a.Id()?, 1);
    assert_eq!(a.Status()?, AsyncStatus::Completed);
    assert_eq!(a.ErrorCode()?, HRESULT(0));
    assert_eq!(a.Completed(), Err(Error::empty()));
    assert_eq!(a.GetResults(), Ok(123));
    a.Cancel()?;
    a.Close()?;

    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationCompletedHandler::new(
        move |sender, status| {
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            send.send(()).unwrap();
            Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into()) // handler error ignored
        },
    ))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(&AsyncOperationCompletedHandler::new(move |_, _| { Ok(()) }))
            .unwrap_err()
            .code(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    assert_eq!(
        IAsyncOperation::ready(Ok(HSTRING::from("hello"))).get()?,
        "hello"
    );

    Ok(())
}

#[test]
fn err() -> Result<()> {
    let a = IAsyncOperation::<i32>::ready(Err(Error::new(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
        "async",
    )));
    assert_eq!(a.Status()?, AsyncStatus::Error);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    let error = a.GetResults().unwrap_err();
    assert_eq!(error.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(error.message(), "async");

    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationCompletedHandler::new(
        move |sender, status| {
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Error);
            send.send(()).unwrap();
            Ok(())
        },
    ))?;

    recv.recv().unwrap();
    Ok(())
}
