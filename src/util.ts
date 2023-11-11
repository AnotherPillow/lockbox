export const capitalise = (input: string) => input[0].toUpperCase() + input.substring(1)

export const blankAccount = {
    title: 'Example.com - username@email.site',
    site: 'example.com',
    email: 'username@email.site',
    username: 'username',
    password: 'password123',
    notes: '',
    humanname: 'User Name',
    yearOfBirth: 2000,
    monthOfBirth: 6,
    dayOfBirth: 18,
}

export const randomPassword = (
    length = 15,
    charset = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"£$%^&*()-=_+[{]}#~@;:,<.>/?'
) => {
    let pwd = ''
    for (var i = 0, n = charset.length; i < length; ++i) {
        pwd += charset.charAt(Math.floor(Math.random() * n));
    }
    return pwd
}