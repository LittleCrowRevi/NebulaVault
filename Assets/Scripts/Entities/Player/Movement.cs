using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.Serialization;

public class Movement : MonoBehaviour
{
    // exists to control linear drag/friction during movement
    private bool _isMoving;

    private bool isMoving
    {
        set
        {
            _isMoving = value;
            if ( body )
                body.drag = isMoving switch
                {
                    true  => movingDrag,
                    false => stopDrag
                };
        }
        get => _isMoving;
    }

    [Header( "States" )]
    [SerializeField] private bool canMove = false;

    [Header( "Movement Values" )]
    [SerializeField] private float stopDrag = 50f;

    [SerializeField] private float movingDrag    = 10f;
    [SerializeField] public  float movementSpeed = 5000.0F;

    private Vector2     direction = Vector2.zero;
    private Rigidbody2D body;

    private void Awake()
    {
        body = gameObject.GetComponent< Rigidbody2D >();
    }

    private void Start()
    {
        canMove = true;
    }

    private void FixedUpdate()
    {
        if ( direction != Vector2.zero && canMove )
        {
            Vector2 velocity = new Vector2( direction.x * movementSpeed * Time.deltaTime, direction.y * movementSpeed * Time.deltaTime );
            if ( body ) body.AddForce( velocity );
            isMoving = true;
        }
        else
        {
            isMoving = false;
        }
    }

    private void changeMovementState()
    {
        canMove = !canMove;
    }

    public void OnMove( InputValue value )
    {
        direction = value.Get< Vector2 >();
    }

    private void OnTriggerEnter2D( Collider2D collision ) => OnCollisionHandle( collision );

    private void OnCollisionHandle( Collider2D collision )
    {
        //Debug.Log(collision);
    }
}