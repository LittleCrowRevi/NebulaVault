using System;
using Unity.VisualScripting;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.Serialization;

public class CameraComponent : MonoBehaviour
{
    [Header( "Camera Data" )]
    [SerializeField] private float updateSpeed = 0.2f;

    [SerializeField] private GameObject trackedEntity;

    private float   UpdateSpeed => updateSpeed;
    private Vector3 _velocity     = Vector3.zero;
    private Vector3 trackedOffset = Vector3.zero;

    [Header( "Listen to Events" )]
    [SerializeField] private GameObjectEventChannelSO m_ChangeCameraTarget;

    private void Start()
    {
        if ( m_ChangeCameraTarget ) m_ChangeCameraTarget.OnRaiseEvent += OnChangeCameraTarget;
    }

    private void FixedUpdate()
    {
        if ( trackedEntity )
        {
            transform.position = Vector3.SmoothDamp( transform.position, trackedEntity.transform.position + trackedOffset, ref _velocity, UpdateSpeed );
        }
    }

    private void OnChangeCameraTarget( GameObject newTarget )
    {
        trackedEntity   = newTarget;
        _velocity       = trackedEntity.GetComponent< Rigidbody2D >().velocity;
        trackedOffset.z = transform.position.z - trackedEntity.transform.position.z;
    }
}